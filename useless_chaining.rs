#![macro_use]
use middle::analysis::ast::*;
use ast::Expression::*;

pub struct UselessChaining<'a: 'c, 'b: 'a, 'c>
{
  grammar: &'c AGrammar<'a, 'b>,
  not_not: usize,
  and_and: (usize,usize),
  not_and: usize,
  and_not: usize,
  oom_oom: (usize,usize),
  oom_zom: usize
}

impl <'a, 'b, 'c> UselessChaining<'a, 'b, 'c>
{
  pub fn analyse(grammar: AGrammar<'a, 'b>) -> Partial<AGrammar<'a, 'b>> {
    UselessChaining::check_chaining(&grammar);
    Partial::Nothing
  }

  fn check_chaining(grammar: &'c AGrammar<'a, 'b>){
    let mut analyser = UselessChaining{
      grammar: grammar,
      not_not: 0,
      and_and: (0,0),
      not_and: 0,
      and_not: 0,
      oom_oom: (0,0),
      oom_zom: 0
  };
    for rule in &grammar.rules {
      println!("\nRegle");
      analyser.visit_expr(rule.expr_idx);
    }
  }

  fn clear_vars(&mut self){
      self.verify_and_and();
      self.verify_oom_oom();
      self.not_not = 0;
      self.and_and = (0,0);
      self.not_and = 0;
      self.and_not = 0;
      self.oom_oom = (0,0);
      self.oom_zom = 0
  }

  fn verify_and_and(&mut self){
      let (cpt_and_and, span_and_and) = self.and_and;
      if cpt_and_and>=2 {
          self.grammar.cx.span_warn(self.grammar[span_and_and].span(), "Detected useless chaining: multiple & \n Help: &(&e) -> &e");
      }
  }

  fn verify_oom_oom(&mut self){
      let (cpt_oom_oom, span_oom_oom) = self.oom_oom;
      if cpt_oom_oom>=2 {
          self.grammar.cx.span_warn(self.grammar[span_oom_oom].span(), "Detected useless chaining: multiple + \n Help: (e+)+ -> e+");
      }
  }
}

impl<'a, 'b, 'c> ExprByIndex for UselessChaining<'a, 'b, 'c>
{
  fn expr_by_index(&self, index: usize) -> Expression {
    self.grammar.expr_by_index(index).clone()
  }
}

impl<'a, 'b, 'c> Visitor<()> for UselessChaining<'a, 'b, 'c>
{
    unit_visitor_impl!(str_literal);
    unit_visitor_impl!(atom);
    unit_visitor_impl!(sequence);
    unit_visitor_impl!(choice);

  fn visit_expr(&mut self, this: usize) {
      match self.expr_by_index(this) {
        NonTerminalSymbol(rule) => {
          self.visit_non_terminal_symbol(this, rule)
        }
        ZeroOrMore(child) => {
          self.visit_zero_or_more(this, child)
        }
        OneOrMore(child) => {
          self.visit_one_or_more(this, child)
        }
        NotPredicate(child) => {
          self.visit_not_predicate(this, child)
        }
        AndPredicate(child) => {
          self.visit_and_predicate(this, child)
        }
        _ => {
            self.clear_vars()
        }
      }
  }

  fn visit_non_terminal_symbol(&mut self, _this: usize, rule: Ident){
    self.visit_expr(self.grammar.find_rule_by_ident(rule).expr_idx)
  }

  fn visit_one_or_more(&mut self, this: usize, child: usize){
    println!("one_or_more");
    let (nb, _) = self.oom_oom;
    self.oom_oom = (nb+1, this);
    self.oom_zom+=1;
    self.not_not = 0;
    self.verify_and_and();
    self.and_and = (0,0);
    self.not_and = 0;
    self.and_not = 0;
    self.visit_expr(child)
  }

  fn visit_zero_or_more(&mut self, this: usize, child: usize){
    println!("zero_or_more");
    if self.oom_zom+1>=2 {
        self.grammar.cx.span_warn(self.grammar[this].span(), "Detected useless chaining: (e+)* \nHelp: (e+)* -> e+");
    }
    self.not_not = 0;
    self.verify_and_and();
    self.and_and = (0,0);
    self.not_and = 0;
    self.and_not = 0;
    self.verify_oom_oom();
    self.oom_oom = (0,0);
    self.oom_zom = 0;
    self.visit_expr(child)
  }

  fn visit_not_predicate(&mut self, this: usize, child: usize){
    println!("not_predicate");
    if self.not_not+1>=2 {
        self.grammar.cx.span_warn(self.grammar[this].span(), "Detected useless chaining: !(!e) \nHelp: !(!e) -> &e");
    }
    if self.and_not+1>=2 {
        self.grammar.cx.span_warn(self.grammar[this].span(), "Detected useless chaining: &(!e) \nHelp: &(!e) -> !e");
    }
    self.not_not+=1;
    self.not_and+=1;
    self.verify_and_and();
    self.and_and = (0,0);
    self.and_not = 0;
    self.verify_oom_oom();
    self.oom_oom = (0,0);
    self.oom_zom = 0;
    self.visit_expr(child)
  }

  fn visit_and_predicate(&mut self, this: usize, child: usize){
    println!("and_predicate");
    if self.not_and+1>=2 {
        self.grammar.cx.span_warn(self.grammar[this].span(), "Detected useless chaining: !(&e) \n Help: !(&e) -> !e");
    }
    self.grammar.cx.span_warn(self.grammar[this-1].span(), "this is an and");
    let (nb, _) = self.and_and;
    self.and_and = (nb+1, this);
    self.and_not+=1;
    self.not_not = 0;
    self.not_and = 0;
    self.verify_oom_oom();
    self.oom_oom = (0,0);
    self.oom_zom = 0;
    self.visit_expr(child)
  }
}
