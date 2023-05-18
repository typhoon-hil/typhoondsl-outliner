///! This file is maintained by rustemo but can be modified manually.
///! All manual changes will be preserved except non-doc comments.
use serde::Serialize;
use rustemo::lexer;
use super::outliner::Context;
use super::outliner::TokenKind;
use super::outliner_lexer::Input;
#[allow(dead_code)]
pub type Token<'i> = lexer::Token<'i, Input, TokenKind>;
pub type OBrace = ();
pub fn obrace<'i>(_ctx: &Context<'i>, _token: Token<'i>) -> OBrace {}
pub type CBrace = ();
pub fn cbrace<'i>(_ctx: &Context<'i>, _token: Token<'i>) -> CBrace {}
pub type ComponentKW = ();
pub fn component_kw<'i>(_ctx: &Context<'i>, _token: Token<'i>) -> ComponentKW {}
pub type ConfigurationKW = ();
pub fn configuration_kw<'i>(_ctx: &Context<'i>, _token: Token<'i>) -> ConfigurationKW {}
pub type CodeKW = ();
pub fn code_kw<'i>(_ctx: &Context<'i>, _token: Token<'i>) -> CodeKW {}
pub type EndCodeKW = ();
pub fn end_code_kw<'i>(_ctx: &Context<'i>, _token: Token<'i>) -> EndCodeKW {}
pub type ModelKW = ();
pub fn model_kw<'i>(_ctx: &Context<'i>, _token: Token<'i>) -> ModelKW {}
pub type ID = String;
pub fn id<'i>(_ctx: &Context<'i>, token: Token<'i>) -> ID {
    token.value.into()
}
pub type TillEndCodeKW = ();
pub fn till_end_code_kw<'i>(_ctx: &Context<'i>, _token: Token<'i>) -> TillEndCodeKW {}
pub type Name = String;
pub fn name<'i>(_ctx: &Context<'i>, token: Token<'i>) -> Name {
    token.value.into()
}
#[derive(Debug, Clone, Serialize)]
pub struct Location {
    line_start: usize,
    column_start: usize,
    line_end: usize,
    column_end: usize
}
impl From<rustemo::location::Location> for Location {
    fn from(location: rustemo::location::Location) -> Self {
        let (line_start, column_start) = match location.start {
            rustemo::location::Position::LineBased(lb) => (lb.line, lb.column),
            rustemo::location::Position::Position(_) => panic!("Position must be line/column based."),
        };
        let (line_end, column_end) = match location.end.expect("End position must be set!") {
            rustemo::location::Position::LineBased(lb) => (lb.line, lb.column),
            rustemo::location::Position::Position(_) => panic!("Position must be line/column based."),
        };
        Location {
            line_start,
            column_start,
            line_end,
            column_end,
        }
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct Model {
    pub name: Name,
    pub location: Location,
    pub configuration: Configuration,
    pub elements: Element0,
}
pub fn model_c1(
    ctx: &Context,
    _model_kw: ModelKW,
    name: Name,
    _obrace: OBrace,
    configuration: Configuration,
    elements: Element0,
    _cbrace: CBrace,
) -> Model {
    Model {
        name,
        location: ctx.location.into(),
        configuration,
        elements,
    }
}
#[derive(Debug, Clone, Serialize)]
pub enum Element {
    Component(Component),
    Handler(Handler),
    Block,
}
pub fn element_component(_ctx: &Context, component: Component) -> Element {
    Element::Component(component)
}
pub fn element_handler(_ctx: &Context, handler: Handler) -> Element {
    Element::Handler(handler)
}
pub fn element_block(_ctx: &Context, _block: Block) -> Element {
    Element::Block
}
pub type Element0 = Option<Element1>;
pub fn element0_element1(_ctx: &Context, element1: Element1) -> Element0 {
    Some(element1)
}
pub fn element0_empty(_ctx: &Context) -> Element0 {
    None
}
pub type Element1 = Vec<Element>;
pub fn element1_c1(
    _ctx: &Context,
    mut element1: Element1,
    element: Element,
) -> Element1 {
    if let Element::Component(_) | Element::Handler(_) = element {
        element1.push(element)
    }
    element1
}
pub fn element1_element(_ctx: &Context, element: Element) -> Element1 {
    let mut v = vec![];
    if let Element::Component(_) | Element::Handler(_) = element {
        v.push(element)
    }
    v
}
#[derive(Debug, Clone, Serialize)]
pub struct Component {
    pub component_type: TypeName,
    pub name: Name,
    pub idopt: IDOpt,
    pub elements: Box<Element0>,
    pub location: Location,
}
pub fn component_c1(
    ctx: &Context,
    _component_kw: ComponentKW,
    component_type: TypeName,
    name: Name,
    idopt: IDOpt,
    _obrace: OBrace,
    elements: Element0,
    _cbrace: CBrace,
) -> Component {
    Component {
        component_type,
        name,
        idopt,
        elements: Box::new(elements),
        location: ctx.location.into(),
    }
}
pub type IDOpt = Option<ID>;
pub fn idopt_id(_ctx: &Context, id: ID) -> IDOpt {
    Some(id)
}
pub fn idopt_empty(_ctx: &Context) -> IDOpt {
    None
}
#[derive(Debug, Clone, Serialize)]
pub struct Configuration {
    pub location: Location,
}
pub fn configuration_c1(
    ctx: &Context,
    _configuration_kw: ConfigurationKW,
    _block: Block,
) -> Configuration {
    Configuration {
        location: ctx.location.into(),
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct Block {
    pub element0: Element0,
}
pub fn block_c1(
    _ctx: &Context,
    _obrace: OBrace,
    element0: Element0,
    _cbrace: CBrace,
) -> Block {
    Block { element0 }
}
#[derive(Debug, Clone, Serialize)]
pub struct Handler {
    pub name: ID,
    pub location: Location,
}
pub fn handler_c1(
    ctx: &Context,
    _code_kw: CodeKW,
    name: ID,
    _till_end_code_kwopt: TillEndCodeKWOpt,
    _end_code_kw: EndCodeKW,
) -> Handler {
    Handler {
        name,
        location: ctx.location.into(),
    }
}
pub type TillEndCodeKWOpt = ();
pub fn till_end_code_kwopt_till_end_code_kw(
    _ctx: &Context,
    _till_end_code_kw: TillEndCodeKW,
) -> TillEndCodeKWOpt {}
pub fn till_end_code_kwopt_empty(_ctx: &Context) -> TillEndCodeKWOpt {}
#[derive(Debug, Clone, Serialize)]
pub enum TypeName {
    Name(Name),
    ID(ID),
}
pub fn type_name_name(_ctx: &Context, name: Name) -> TypeName {
    TypeName::Name(name)
}
pub fn type_name_id(_ctx: &Context, id: ID) -> TypeName {
    TypeName::ID(id)
}
