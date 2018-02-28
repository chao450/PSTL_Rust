#![feature(plugin)]
#![plugin(oak)]

extern crate oak_runtime;
use oak_runtime::*;


grammar! json {
    #![show_api]
    digit = ["0-9"]
    id = ["a-zA-Z0-9_() "]
    colon = ":" spacing
    vir = "," spacing
    crochetg = "[" spacing
    crochetd = "]" spacing
    guil = "\""
    accolg = "{" spacing
    accold = "}" spacing

    number = digit+ spacing > to_number
    string = guil id+ guil spacing > to_string
    array = crochetg (json_expr vir)* json_expr spacing crochetd
    object = accolg inst? spacing accold

    spacing = [" \n\r\t"]* -> (^)


    use std::str::FromStr;

    program = spacing json_expr spacing


    couple = string colon json_expr > make_json_couple
    inst = couple (vir couple)* > make_json_inst

    json_expr =
        number > make_json_number
    /   string > make_json_string
    /   array > make_json_array
    /   object > make_json_object



    pub type PExpr = Box<JSONExpr>;

    #[derive(Debug)]
    pub enum JSONExpr {
        Str(String),
        Number(u32),
        Array(Vec<Box<JSONExpr>>),
        Object(Option<Box<JSONCouple>>)
    }

    #[derive(Debug)]
    pub enum JSONCouple {
        Couple(String, Box<JSONExpr>),
        Inst(Vec<Box<JSONCouple>>)
    }

    fn to_number(raw_text: Vec<char>) -> u32 {
        u32::from_str(&*to_string(raw_text)).unwrap()
    }

    fn to_string(raw_text: Vec<char>) -> String {
        raw_text.into_iter().collect()
    }

    fn make_json_number(number:u32)-> Box<JSONExpr> {
        Box::new(JSONExpr::Number(number))
    }

    fn make_json_string(string:String) -> Box<JSONExpr> {
        Box::new(JSONExpr::Str(string))
    }

    fn make_json_couple(string:String, expr:Box<JSONExpr>) -> Box<JSONCouple> {
        Box::new(JSONCouple::Couple(string,expr))
    }

    fn make_json_array(array:Vec<Box<JSONExpr>>, front:Box<JSONExpr>) -> Box<JSONExpr> {
        let mut v = Vec::new();
        for i in array{
            v.push(i);
        }
        v.push(front);
        Box::new(JSONExpr::Array(v))
    }

    fn make_json_inst(cp: Box<JSONCouple>, rest: Vec<Box<JSONCouple>>) -> Box<JSONCouple> {
        let mut v = vec![cp];
        for i in rest{
            v.push(i);
        }
        Box::new(JSONCouple::Inst(v))
    }

    fn make_json_object(inst: Option<Box<JSONCouple>>) -> Box<JSONExpr> {
        Box::new(JSONExpr::Object(inst))
    }
}

fn analyse_state(state: ParseState<StrStream, json::PExpr>)  {
    use oak_runtime::parse_state::ParseResult::*;
    match state.into_result() {
        Success(data) => println!("Full match: {:?}", data),
        Partial(data, expectation) => {
            println!("Partial match: {:?} because {:?}", data, expectation);
        }
        Failure(expectation) => {
            println!("Failure: {:?}", expectation);
        }
    }
}

fn main() {
    analyse_state(json::parse_program("{\"age\" : 6}".into_state())); // Complete
    analyse_state(json::parse_program("{\"age\" : 6},".into_state())); // Partial
    analyse_state(json::parse_program("{: 6}".into_state())); // Error
    analyse_state(json::parse_program("{}".into_state()));

    let prog =
        r##"
        {
            "menu": {
                "id" : "file",
                "value" : "File",
                "popup" : {
                    "menuitem" : [
                        { "value" : "New", "onclick" : "CreateNewDoc()" },
                        { "value" : "Open", "onclick" : "OpenDoc()"},
                        { "value" : "Close", "onclick" : "CloseDoc()"}
                    ]
                }
            }
        }
        "##;
    analyse_state(json::parse_program(prog.into_state()));
}
