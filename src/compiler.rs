use crate::ast::*;
use crate::ast_walker::{ast_walker, AstVisitor};
use crate::func::{Proto, ProtoContext};

pub struct Compiler {
    proto_contexts: Vec<ProtoContext>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            proto_contexts: Vec::new(),
        }
    }

    pub fn run(&mut self, block: &Block) -> Proto {
        self.main_func(block)
    }

    fn main_func(&mut self, block: &Block) -> Proto {
        self.push_proto();
        self.proto().open();
        ast_walker::walk_block(block, self);
        self.proto().close();
        self.pop_proto()
    }

    fn push_proto(&mut self) {
        self.proto_contexts.push(ProtoContext::new());
    }

    fn pop_proto(&mut self) -> Proto {
        if let Some(context) = self.proto_contexts.pop() {
            return context.proto;
        }
        unreachable!()
    }

    // get current proto ref from stack
    fn proto(&mut self) -> &mut Proto {
        &mut self.context().proto
    }

    // get current proto context
    fn context(&mut self) -> &mut ProtoContext {
        if let Some(last) = self.proto_contexts.last_mut() {
            return last;
        }
        unreachable!()
    }

    fn adjust_assign(&mut self, names: &Vec<String>, exprs: &Vec<Expr>) {
        let extra = names.len() as i32 - exprs.len() as i32;
        if let Some(last_expr) = exprs.last() {
            if last_expr.has_mult_ret() {
                todo!("process mult ret")
            }
        }

        if extra > 0 {
            let context = self.context();
            let from = context.free_reg;
            context.reverse_regs(extra as u32);
            context.proto.code_nil(from, extra as u32);
        }
    }
}

impl AstVisitor for Compiler {
    fn local_stat(&mut self, stat: &LocalStat) {
        let proto = self.proto();
        for name in stat.names.iter() {
            proto.add_local_var(name);
        }
        ast_walker::walk_exprlist(&stat.exprs, self);
        self.adjust_assign(&stat.names, &stat.exprs);
    }
}
