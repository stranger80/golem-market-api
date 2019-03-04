extern crate nom;
extern crate asnom;
extern crate market_api;

use market_api::resolver::*;
use market_api::resolver::properties::*;
use market_api::resolver::ldap_parser::parse;
use market_api::resolver::expression::*;

fn run_resolve_test(expr : &str, props : &Vec<&str>, expect_result : ResolveResult) {
    let expression = build_expression(&parse(expr).unwrap()).unwrap();

    let mut properties = vec![];
    for prop in props {
        properties.push(prop.to_string());
    }

    let property_set = PropertySet::from_flat_props(&properties);

    assert_eq!(expression.resolve(&property_set), expect_result);
}

#[test]
fn resolve_empty() {
    let f = "()";

    // test positive 

    run_resolve_test(f, &vec!["objectClass=\"Babs Jensen\""], ResolveResult::True);
}

#[test]
fn resolve_present() {
    let f = "(objectClass=*)";

    // test positive 

    run_resolve_test(f, &vec!["objectClass=\"Babs Jensen\""], ResolveResult::True);

    // test negative (must return name of unresolved property)

    run_resolve_test(f, &vec!["cn=\"Dblah\""], 
                    ResolveResult::False(
                        vec![&PropertyRef::Value(String::from("objectClass"), PropertyRefType::Any)],
                        Expression::Empty
                    ));
}

#[test]
fn resolve_equals() {
    let f = "(cn=Babs Jensen)";

    // test positive

    run_resolve_test(f, &vec!["cn=\"Babs Jensen\""], ResolveResult::True);

    // test negative

    run_resolve_test(f, &vec!["cn=\"Dblah\""], 
                    ResolveResult::False(
                        vec![],
                        Expression::Empty
                    ));

    // test undefined

    run_resolve_test(f, &vec!["cnas=\"Dblah\""], 
                    ResolveResult::Undefined(
                        vec![&PropertyRef::Value(String::from("cn"), PropertyRefType::Any)],
                        Expression::Equals(PropertyRef::Value(String::from("cn"), PropertyRefType::Any), String::from("Babs Jensen"))
                    ));
}

#[test]
fn resolve_equals_with_wildcard() {
    let f = "(cn=Babs *)";

    // test positive

    run_resolve_test(f, &vec!["cn=\"Babs Jensen\""], ResolveResult::True);

    // test negative

    run_resolve_test(f, &vec!["cn=\"Dblah\""], 
                    ResolveResult::False(
                        vec![],
                        Expression::Empty
                    ));

    // test undefined

    run_resolve_test(f, &vec!["cnas=\"Dblah\""], 
                    ResolveResult::Undefined(
                        vec![&PropertyRef::Value(String::from("cn"), PropertyRefType::Any)],
                        Expression::Equals(PropertyRef::Value(String::from("cn"), PropertyRefType::Any), String::from("Babs *"))
                    ));
}

#[test]
fn resolve_equals_int() {
    let f = "(cn=123)";

    // test positive

    run_resolve_test(f, &vec!["cn=123"], ResolveResult::True);

    // test negative

    run_resolve_test(f, &vec!["cn=456"], 
                    ResolveResult::False(
                        vec![],
                        Expression::Empty
                    ));

    // test false when parsing error

    let f = "(cn=1ds23)";
    run_resolve_test(f, &vec!["cn=123"], 
                    ResolveResult::False(
                        vec![],
                        Expression::Empty
                    ));

}

#[test]
fn resolve_greater_int() {
    let f = "(cn>123)";

    // test positive

    run_resolve_test(f, &vec!["cn=124"], ResolveResult::True);

    // test negative

    run_resolve_test(f, &vec!["cn=12"], 
                    ResolveResult::False(
                        vec![],
                        Expression::Empty
                    ));

    // test false when parsing error

    let f = "(cn>1ds23)";
    run_resolve_test(f, &vec!["cn=123"], 
                    ResolveResult::False(
                        vec![],
                        Expression::Empty
                    ));

}

#[test]
fn resolve_less_float() {
    let f = "(cn<123.56)";

    // test positive

    run_resolve_test(f, &vec!["cn=122.674"], ResolveResult::True);

    // test negative

    run_resolve_test(f, &vec!["cn=126"], 
                    ResolveResult::False(
                        vec![],
                        Expression::Empty
                    ));

    // test false when parsing error

    let f = "(cn<1ds23)";
    run_resolve_test(f, &vec!["cn=123"], 
                    ResolveResult::False(
                        vec![],
                        Expression::Empty
                    ));

}

#[test]
fn resolve_less_equal_datetime() {
    let f = "(cn<=1985-04-12T23:20:50.52Z)";

    // test positive

    run_resolve_test(f, &vec!["cn=t\"1985-04-12T23:20:30.52Z\""], ResolveResult::True);
    run_resolve_test(f, &vec!["cn=t\"1985-04-12T23:20:50.52Z\""], ResolveResult::True);

    // test negative

    run_resolve_test(f, &vec!["cn=t\"1985-04-12T23:21:50.52Z\""], 
                    ResolveResult::False(
                        vec![],
                        Expression::Empty
                    ));

    // test false when parsing error (NOTE the RFC 3339 format is fairly strict)

    let f = "(cn<=1985-04-13)";
    run_resolve_test(f, &vec!["cn=t\"1985-04-12T23:20:50.52Z\""], 
                    ResolveResult::False(
                        vec![],
                        Expression::Empty
                    ));

}


#[test]
fn resolve_greater_equal_version_with_implied_type() {

    // test positive

    run_resolve_test("(cn@v>=1.5.0)", &vec!["cn=\"1.10.0\""], ResolveResult::True);

    // test negative

    run_resolve_test("(cn>=1.5.0)", &vec!["cn=\"1.10.0\""], ResolveResult::False(vec![], Expression::Empty));

    // test - unable to convert

    run_resolve_test("(cn@v>=1.5.0)", &vec!["cn=\"dblah\""], ResolveResult::Undefined(vec![], 
                    Expression::GreaterEqual(
                        PropertyRef::Value(
                            String::from("cn"), 
                            PropertyRefType::Version), 
                        String::from("1.5.0") 
                    )
                ));

}

#[test]
fn resolve_greater_equal_decimal_with_implied_type() {

    // test positive

    run_resolve_test("(cn@d>=10)", &vec!["cn=\"1\""], ResolveResult::False(vec![], Expression::Empty));

    // test - unable to convert

    run_resolve_test("(cn@d>=10)", &vec!["cn=\"dblah\""], ResolveResult::Undefined(vec![], 
                    Expression::GreaterEqual(
                        PropertyRef::Value(
                            String::from("cn"), 
                            PropertyRefType::Decimal), 
                        String::from("10") 
                    )
                ));

}

#[test]
fn resolve_not() {
    let f = "(!(cn=Tim Howes))";

    // test positive

    run_resolve_test(f, &vec!["cn=\"Babs Jensen\""], ResolveResult::True);

    // test negative

    run_resolve_test(f, &vec!["cn=\"Tim Howes\""], 
                    ResolveResult::False(
                        vec![],
                        Expression::Empty
                    ));

    // test undefined

    run_resolve_test(f, &vec!["cnas=\"Dblah\""], 
                    ResolveResult::Undefined(
                        vec![&PropertyRef::Value(String::from("cn"), PropertyRefType::Any)],
                        Expression::Not(Box::new(Expression::Equals(PropertyRef::Value(String::from("cn"), PropertyRefType::Any), String::from("Tim Howes"))))
                    ));
}

#[test]
fn resolve_and() {
    let f = "(&(a=b)(b=c)(c=d))";

    // test positive

    run_resolve_test(f, &vec!["a=\"b\"", "b=\"c\"", "c=\"d\""], ResolveResult::True);

    // test negative

    run_resolve_test(f, &vec!["a=\"x\"", "b=\"c\"", "c=\"d\""], 
                    ResolveResult::False(
                        vec![],
                        Expression::Empty
                    ));

    // test undefined

    run_resolve_test(f, &vec!["b=\"c\"", "c=\"d\""], 
                    ResolveResult::Undefined(
                        vec![&PropertyRef::Value(String::from("a"), PropertyRefType::Any)],
                        Expression::And(vec![Box::new(Expression::Equals(PropertyRef::Value(String::from("a"), PropertyRefType::Any), String::from("b")))])
                    ));
}



#[test]
fn resolve_or() {
    let f = "(|(a=b)(b=c)(c=d))";

    // test positive

    run_resolve_test(f, &vec!["a=\"b\"", "b=\"c\"", "c=\"d\""], ResolveResult::True);

    // test negative

    run_resolve_test(f, &vec!["a=\"x\"", "b=\"y\"", "c=\"z\""], 
                    ResolveResult::False(
                        vec![],
                        Expression::Empty
                    ));

    // test undefined

    run_resolve_test(f, &vec!["b=\"x\"", "c=\"y\""], 
                    ResolveResult::Undefined(
                        vec![&PropertyRef::Value(String::from("a"), PropertyRefType::Any)],
                        Expression::Or(vec![Box::new(Expression::Equals(PropertyRef::Value(String::from("a"), PropertyRefType::Any), String::from("b")))])
                    ));
}

#[test]
fn resolve_complex() {
    let f = "(&(|(a=b)(b=c)(c=d))(!(x=dblah)))";

    // test positive

    run_resolve_test(f, &vec!["a=\"b\"", "b=\"x\"", "c=\"y\"", "x=\"notdblah\""], ResolveResult::True);
}

#[test]
fn resolve_complex_or_undefined() {
    let f = "(|(a=b)(b=c)(c=d))";

    // test positive

    run_resolve_test(f, &vec![/*"a=\"b\"",*/ "b=\"x\"", "c=\"y\"", "x=\"notdblah\""], 
                    ResolveResult::Undefined(
                        vec![&PropertyRef::Value(String::from("a"), PropertyRefType::Any)],
                        Expression::Or(vec![Box::new(Expression::Equals(PropertyRef::Value(String::from("a"), PropertyRefType::Any), String::from("b")))])
                    ));
}


#[test]
fn resolve_complex_or_undefined_reduced() {
    let f = "(|(a=b)(b=c)(c=d))";

    // test positive

    run_resolve_test(f, &vec![/*"a=\"b\"",*/ "b=\"c\"", "c=\"y\"", "x=\"notdblah\""], 
                    ResolveResult::True);
}

#[test]
fn resolve_complex_and_undefined() {
    let f = "(&(a=b)(b=c)(c=d))";

    // test positive

    run_resolve_test(f, &vec![/*"a=\"b\"",*/ "b=\"c\"", "c=\"d\"", "x=\"notdblah\""], 
                    ResolveResult::Undefined(
                        vec![&PropertyRef::Value(String::from("a"), PropertyRefType::Any)],
                        Expression::And(vec![Box::new(Expression::Equals(PropertyRef::Value(String::from("a"), PropertyRefType::Any), String::from("b")))])
                    ));
}

#[test]
fn resolve_complex_and_undefined_reduced() {
    let f = "(&(a=b)(b=c)(c=d))";

    // test positive

    run_resolve_test(f, &vec![/*"a=\"b\"",*/ "b=\"c\"", "c=\"x\"", "x=\"notdblah\""], 
                    ResolveResult::False(
                        vec![],
                        Expression::Empty
                    ));
}
