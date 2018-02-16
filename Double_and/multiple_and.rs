#![macro_use]
use middle::analysis::ast::*;
use ast::Expression::*;


pub struct MultipleAnd<'a: 'c, 'b: 'a, 'c>
{
  grammar: &'c AGrammar<'a, 'b> 
}

impl <'a, 'b, 'c> MultipleAnd<'a, 'b, 'c>
{
  pub fn analyse(grammar: AGrammar<'a, 'b>) -> Partial<AGrammar<'a, 'b>> {
      if MultipleAnd::detected_multiple_and(&grammar){
          grammar.warn(format!("Multiple And"));
      }
      Partial::Nothing
  }

  fn detected_multiple_and(grammar: &'a AGrammar<'a, 'b>) -> bool {
      let mut cpt = 0;
      let mut detected = false;
    /*let mut analyser = MultipleAnd {
      grammar: grammar
  };*/
    // &grammar.rules ?
    for expr in &grammar.exprs {
      match expr {
          &AndPredicate(_) => cpt = cpt+1,
          _ => if cpt>=2 {
                detected = true;
                cpt = 0;
               },
      }

      match expr {
          &StrLiteral(_) => println!("StrLiteral"),
          &AnySingleChar => println!("AnySingleChar"),
          &CharacterClass(_) => println!("CharacterClass"),
          &NonTerminalSymbol(_) => println!("NonTerminalSymbol -> à évaluer en terminal symbol"),
          &Sequence(_) => println!("Sequence"),
          &Choice(_) => println!("Choice"),
          &ZeroOrMore(_) => println!("ZeroOrOne"),
          &OneOrMore(_) => println!("OneOrMore"),
          &ZeroOrOne(_) => println!("ZeroOrOne"),
          &NotPredicate(_) => println!("NotPredicate"),
          &AndPredicate(_) => println!("AndPredicate"),
          &SemanticAction(_, _) => println!("SemanticAction"),
          &TypeAscription(_, _) => println!("TypeAscription"),
          &SpannedExpr(_) => println!("SpannedExpr"),
      }
    }
    detected
  }
}
/*
impl<'a, 'b, 'c> Visitor<()> for MultipleAnd<'a, 'b, 'c>
{
  /*unit_visitor_impl!(str_literal);
  unit_visitor_impl!(atom);
  unit_visitor_impl!(sequence);
  unit_visitor_impl!(choice);
  unit_visitor_impl!(non_terminal);
  */
  fn visit_and_predicate(&mut self, this: usize, child: usize){
    println!("and predicate detected")
  }
}*/
