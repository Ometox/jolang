use jolang_shared::ir::instructions::Instruction;
use crate::scope::{Scope, ScopeKind};
use crate::generator::{Generate, IrGenerator};
use super::{Expr, Program, Statement};

impl Generate for Program {
    fn generate(&self, generator : &mut IrGenerator) {
        let blk = generator.append_block();
        let exit_block = generator.append_block();
        generator.goto_begin(exit_block);
        let code = generator.add(Instruction::Iconst(0)).unwrap();
        generator.add(Instruction::Reti(code));
        let scope = Scope::new(ScopeKind::Root, blk, exit_block);
        generator.enter_scope(scope);
        generator.goto_begin(blk);
        for s in &self.0 {
            s.generate(generator);
        }
        generator.exit_scope();
        generator.add(Instruction::Br(exit_block));
    }
}

impl Generate for Statement {
    fn generate(&self, generator : &mut IrGenerator) {
        match self {
            Self::Block(stmts) => {
                let block = generator.append_block();
                generator.add(Instruction::Br(block));
                generator.goto_begin(block);
                let after_block = generator.append_block();
                let scope = Scope::new(ScopeKind::Block, block, after_block);
                generator.enter_scope(scope);
                for s in stmts {
                    s.generate(generator);
                }
                generator.add(Instruction::Br(after_block));
                generator.exit_scope();
            },
            Self::If(expr, then, _else) => {
                expr.generate(generator);
                let cond = generator.get_current_block().unwrap().last_index();
                let then_block = generator.append_block();
                let else_block = generator.append_block();
                let after_block = match _else {
                    Some(_) => generator.append_block(),
                    None => else_block
                };
                generator.add(Instruction::Briz(else_block, then_block, cond));
                generator.goto_begin(then_block);
                match **then {
                    Self::Block(_) => then.generate(generator),
                    _ => {
                        let scope = Scope::new(ScopeKind::Block, then_block, after_block);
                        generator.enter_scope(scope);
                        then.generate(generator);
                        generator.exit_scope();
                    }
                }
                generator.add(Instruction::Br(after_block));
                generator.goto_begin(else_block);
                if let Some(_else) = _else {
                    match **_else {
                        Self::Block(_) => then.generate(generator),
                        _ => {
                            let scope = Scope::new(ScopeKind::Block, else_block, after_block);
                            generator.enter_scope(scope);
                            _else.generate(generator);
                            generator.exit_scope();
                        }
                    }
                    generator.add(Instruction::Br(after_block));
                    generator.goto_begin(after_block);
                }
            },
            Statement::While(expr, body) => {
                let while_cond = generator.append_block();
                generator.add(Instruction::Br(while_cond));
                let while_body = generator.append_block();
                let after_block = generator.append_block();
                let scope = Scope::new(ScopeKind::Loop, while_cond, after_block);
                generator.enter_scope(scope);
                expr.generate(generator);
                let cond = generator.get_current_block().unwrap().last_index();
generator.add(Instruction::Briz(after_block, while_body, cond));
                generator.goto_begin(while_body);
                body.generate(generator);
                generator.add(Instruction::Br(while_cond));
                generator.exit_scope();
                generator.goto_begin(after_block);
            },
            Self::Loop(body) => {
                let loop_body = generator.append_block();
                let after_block = generator.append_block();
                let scope = Scope::new(ScopeKind::Loop, loop_body, after_block);
                generator.enter_scope(scope);
                generator.goto_begin(loop_body);
                body.generate(generator);
                generator.add(Instruction::Br(after_block));
                generator.goto_begin(after_block);
            },
            Self::Return(expr) => {
                expr.generate(generator);
                let value = generator.get_current_block().unwrap().last_index();
                generator.add(Instruction::Reti(value));
            },
            Self::Continue => {
                generator.add(Instruction::Br(generator.get_scopes()
                    .iter()
                    .filter(|x| x.kind == ScopeKind::Loop)
                    .next()
                    .expect("can't continue outside a loop")
                    .block));
            },
            Self::Break => {
                generator.add(Instruction::Br(generator.get_scopes()
                    .iter()
                    .filter(|x| x.kind == ScopeKind::Loop)
                    .next()
                    .expect("can't continue outside a loop")
                    .exit));
            },
            Self::VarDecl(name, value) => {
                let default_value = match value {
                    Some(Expr::PrimaryExpr(p)) => match p {
                        super::PrimaryExpr::Litteral(v) => v.clone(),
                        _ => 0
                    },
                    _ => 0
                };
                let id = generator.decl_var(name.to_string(), default_value);
                if default_value == 0 && value.is_some() {
                    value.as_ref().unwrap().generate(generator);
                    let val = generator.get_current_block().unwrap().last_index();
                    generator.add(Instruction::Varset(id, val));
                }
            }
            _ => todo!()
        }
    }
}

impl Generate for Expr {
    fn generate(&self, generator : &mut IrGenerator) {
        todo!()
    }
}
