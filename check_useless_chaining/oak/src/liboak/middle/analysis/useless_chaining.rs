#![macro_use]
use middle::analysis::ast::*;
use ast::Expression::*;
pub use rust::NO_EXPANSION;

pub struct UselessChaining<'a: 'c, 'b: 'a, 'c>
{
  grammar: &'c AGrammar<'a, 'b>,
  not_not: Vec<usize>,
  oom_zom: Vec<usize>,
  not_and: Vec<usize>,
  and_not: Vec<usize>,
  oom_oom: Vec<(usize,usize)>,
  and_and: Vec<(usize,usize)>
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
      not_not: vec![],
      and_and: vec![],
      not_and: vec![],
      and_not: vec![],
      oom_oom: vec![],
      oom_zom: vec![]
  };
    for rule in &grammar.rules {
      println!("\nRegle");
      analyser.visit_expr(rule.expr_idx);
    }
  }

  fn clear_vars(&mut self){
      self.verify_and_and();
      self.verify_oom_oom();
      self.not_not.clear();
      self.not_and.clear();
      self.and_not.clear();
      self.oom_zom.clear()
  }

  fn verify_and_and(&mut self){
      if self.and_and.len()>=2 {
          let (first_and , _) = self.and_and.remove(0);
          let (_ , last_and) = self.and_and.pop().expect("error");
          let lo = self.grammar[first_and].span().lo();
          let hi = self.grammar[last_and].span().lo();
          self.grammar.cx.span_warn(
              Span::new(lo,hi,NO_EXPANSION),
              "Detected useless chaining: multiple & \n Help: &(&e) -> &e"
          );
      }
      self.and_and.clear();
  }

  fn verify_oom_oom(&mut self){
      if self.oom_oom.len()>=2 {
          let (first_oom , _) = self.oom_oom.remove(0);
          let (_ , last_oom) = self.oom_oom.pop().expect("error");
          let lo = self.grammar[first_oom].span().hi();
          let hi = self.grammar[last_oom].span().hi();
          self.grammar.cx.span_warn(
              Span::new(lo,hi,NO_EXPANSION),
              "Detected useless chaining: multiple + \n Help: (e+)+ -> e+"
          );
      }
      self.oom_oom.clear();
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
    if !self.oom_zom.is_empty() {
        self.grammar.cx.span_warn(
            Span::new(
                self.grammar[child].span().hi(),
                self.grammar[self.oom_zom.remove(0)].span().hi(),
                NO_EXPANSION
            ),
            "Detected useless chaining: (e+)* \nHelp: (e+)* -> e+"
        );
    }
    self.oom_oom.push((this, child));
    self.verify_and_and();
    self.not_not.clear();
    self.not_and.clear();
    self.and_not.clear();
    self.visit_expr(child)
  }

  fn visit_zero_or_more(&mut self, this: usize, child: usize){
    println!("zero_or_more");
    self.oom_zom.push(this);
    self.verify_and_and();
    self.verify_oom_oom();
    self.not_not.clear();
    self.not_and.clear();
    self.and_not.clear();
    self.visit_expr(child)
  }

  fn visit_not_predicate(&mut self, this: usize, child: usize){
    println!("not_predicate");
    if !self.not_not.is_empty() {
        self.grammar.cx.span_warn(
            Span::new(
                self.grammar[self.not_not.remove(0)].span().lo(),
                self.grammar[child].span().lo(),
                NO_EXPANSION
            ),
            "Detected useless chaining: !(!e) \nHelp: !(!e) -> &e"
        );
    }
    if !self.and_not.is_empty() {
        self.grammar.cx.span_warn(
            Span::new(
                self.grammar[self.and_not.remove(0)].span().lo(),
                self.grammar[child].span().lo(),
                NO_EXPANSION
            ),
            "Detected useless chaining: &(!e) \nHelp: &(!e) -> !e"
        );
    }

    self.not_not.push(this);
    self.not_and.push(this);
    self.verify_and_and();
    self.verify_oom_oom();
    self.and_not.clear();
    self.oom_zom.clear();
    self.visit_expr(child)
  }

  fn visit_and_predicate(&mut self, this: usize, child: usize){
    println!("and_predicate");
    if !self.not_and.is_empty() {
        self.grammar.cx.span_warn(
            Span::new(
                self.grammar[self.not_and.remove(0)].span().lo(),
                self.grammar[child].span().lo(),
                NO_EXPANSION
            ),
            "Detected useless chaining: !(&e) \nHelp: !(&e) -> !e"
        );
    }
    self.and_and.push((this,child));
    self.and_not.push(this);
    self.verify_oom_oom();
    self.not_not.clear();
    self.not_and.clear();
    self.oom_zom.clear();
    self.visit_expr(child)
  }
}
