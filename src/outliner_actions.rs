///! This file is maintained by rustemo but can be modified manually.
///! All manual changes will be preserved except non-doc comments.
use rustemo::lexer;
use super::outliner::Context;
use super::outliner::TokenKind;
use super::outliner_lexer::Input;
#[allow(dead_code)]
pub type Token<'i> = lexer::Token<'i, Input, TokenKind>;
pub type OBrace = String;
pub fn obrace<'i>(_ctx: &Context<'i>, token: Token<'i>) -> OBrace {
    token.value.into()
}
pub type CBrace = String;
pub fn cbrace<'i>(_ctx: &Context<'i>, token: Token<'i>) -> CBrace {
    token.value.into()
}
pub type ComponentKW = String;
pub fn component_kw<'i>(_ctx: &Context<'i>, token: Token<'i>) -> ComponentKW {
    token.value.into()
}
pub type ConfigurationKW = String;
pub fn configuration_kw<'i>(_ctx: &Context<'i>, token: Token<'i>) -> ConfigurationKW {
    token.value.into()
}
pub type CodeKW = String;
pub fn code_kw<'i>(_ctx: &Context<'i>, token: Token<'i>) -> CodeKW {
    token.value.into()
}
pub type EndCodeKW = String;
pub fn end_code_kw<'i>(_ctx: &Context<'i>, token: Token<'i>) -> EndCodeKW {
    token.value.into()
}
pub type ModelKW = String;
pub fn model_kw<'i>(_ctx: &Context<'i>, token: Token<'i>) -> ModelKW {
    token.value.into()
}
pub type ID = String;
pub fn id<'i>(_ctx: &Context<'i>, token: Token<'i>) -> ID {
    token.value.into()
}
pub type TillEndCodeKW = String;
pub fn till_end_code_kw<'i>(_ctx: &Context<'i>, token: Token<'i>) -> TillEndCodeKW {
    token.value.into()
}
pub type Name = String;
pub fn name<'i>(_ctx: &Context<'i>, token: Token<'i>) -> Name {
    token.value.into()
}
#[derive(Debug, Clone)]
pub struct Model {
    pub model_kw: ModelKW,
    pub name: Name,
    pub obrace: OBrace,
    pub configuration: Configuration,
    pub elements: Element0,
    pub cbrace: CBrace,
}
pub fn model_c1(
    _ctx: &Context,
    model_kw: ModelKW,
    name: Name,
    obrace: OBrace,
    configuration: Configuration,
    elements: Element0,
    cbrace: CBrace,
) -> Model {
    Model {
        model_kw,
        name,
        obrace,
        configuration,
        elements,
        cbrace,
    }
}
#[derive(Debug, Clone)]
pub enum Element {
    Component(Component),
    Handler(Handler),
    Block(Box<Block>),
}
pub fn element_component(_ctx: &Context, component: Component) -> Element {
    Element::Component(component)
}
pub fn element_handler(_ctx: &Context, handler: Handler) -> Element {
    Element::Handler(handler)
}
pub fn element_block(_ctx: &Context, block: Block) -> Element {
    Element::Block(Box::new(block))
}
pub type Element0 = Option<Element1>;
pub fn element0_element1(_ctx: &Context, element1: Element1) -> Element0 {
    Some(element1)
}
pub fn element0_empty(_ctx: &Context) -> Element0 {
    None
}
#[derive(Debug, Clone)]
pub struct Element1C1 {
    pub element1: Box<Element1>,
    pub element: Element,
}
#[derive(Debug, Clone)]
pub enum Element1 {
    C1(Element1C1),
    Element(Element),
}
pub fn element1_c1(_ctx: &Context, element1: Element1, element: Element) -> Element1 {
    Element1::C1(Element1C1 {
        element1: Box::new(element1),
        element,
    })
}
pub fn element1_element(_ctx: &Context, element: Element) -> Element1 {
    Element1::Element(element)
}
#[derive(Debug, Clone)]
pub struct Component {
    pub component_kw: ComponentKW,
    pub component_type: TypeName,
    pub name: Name,
    pub idopt: IDOpt,
    pub obrace: OBrace,
    pub elements: Box<Element0>,
    pub cbrace: CBrace,
}
pub fn component_c1(
    _ctx: &Context,
    component_kw: ComponentKW,
    component_type: TypeName,
    name: Name,
    idopt: IDOpt,
    obrace: OBrace,
    elements: Element0,
    cbrace: CBrace,
) -> Component {
    Component {
        component_kw,
        component_type,
        name,
        idopt,
        obrace,
        elements: Box::new(elements),
        cbrace,
    }
}
pub type IDOpt = Option<ID>;
pub fn idopt_id(_ctx: &Context, id: ID) -> IDOpt {
    Some(id)
}
pub fn idopt_empty(_ctx: &Context) -> IDOpt {
    None
}
#[derive(Debug, Clone)]
pub struct Configuration {
    pub configuration_kw: ConfigurationKW,
    pub block: Block,
}
pub fn configuration_c1(
    _ctx: &Context,
    configuration_kw: ConfigurationKW,
    block: Block,
) -> Configuration {
    Configuration {
        configuration_kw,
        block,
    }
}
#[derive(Debug, Clone)]
pub struct Block {
    pub obrace: OBrace,
    pub element0: Element0,
    pub cbrace: CBrace,
}
pub fn block_c1(
    _ctx: &Context,
    obrace: OBrace,
    element0: Element0,
    cbrace: CBrace,
) -> Block {
    Block { obrace, element0, cbrace }
}
#[derive(Debug, Clone)]
pub struct Handler {
    pub code_kw: CodeKW,
    pub name: ID,
    pub till_end_code_kwopt: TillEndCodeKWOpt,
    pub end_code_kw: EndCodeKW,
}
pub fn handler_c1(
    _ctx: &Context,
    code_kw: CodeKW,
    name: ID,
    till_end_code_kwopt: TillEndCodeKWOpt,
    end_code_kw: EndCodeKW,
) -> Handler {
    Handler {
        code_kw,
        name,
        till_end_code_kwopt,
        end_code_kw,
    }
}
pub type TillEndCodeKWOpt = Option<TillEndCodeKW>;
pub fn till_end_code_kwopt_till_end_code_kw(
    _ctx: &Context,
    till_end_code_kw: TillEndCodeKW,
) -> TillEndCodeKWOpt {
    Some(till_end_code_kw)
}
pub fn till_end_code_kwopt_empty(_ctx: &Context) -> TillEndCodeKWOpt {
    None
}
#[derive(Debug, Clone)]
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
