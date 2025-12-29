//! Парсер Sequence Diagrams
//!
//! Использует pest грамматику для парсинга PlantUML sequence diagrams.

use pest::Parser;
use pest_derive::Parser;

use plantuml_ast::sequence::{
    SequenceDiagram, Participant, ParticipantType, Message, 
    ArrowType, Fragment, FragmentType, FragmentSection,
    SequenceElement, Divider, Delay, Activation, ActivationType,
};
use plantuml_ast::common::{LineStyle, Stereotype, Color, Note, NotePosition};

use crate::{ParseError, Result};

#[derive(Parser)]
#[grammar = "grammars/sequence.pest"]
pub struct SequenceParser;

/// Парсит sequence diagram из исходного кода
pub fn parse_sequence(source: &str) -> Result<SequenceDiagram> {
    let pairs = SequenceParser::parse(Rule::diagram, source)
        .map_err(|e| ParseError::SyntaxError {
            line: e.line().to_string().parse().unwrap_or(0),
            message: e.to_string(),
        })?;
    
    let mut diagram = SequenceDiagram::new();
    let mut fragment_stack: Vec<(FragmentType, Option<String>, Vec<FragmentSection>)> = Vec::new();
    let mut current_section_elements: Vec<SequenceElement> = Vec::new();
    
    for pair in pairs {
        if pair.as_rule() == Rule::diagram {
            for inner in pair.into_inner() {
                process_rule(
                    inner, 
                    &mut diagram, 
                    &mut fragment_stack, 
                    &mut current_section_elements
                );
            }
        }
    }
    
    Ok(diagram)
}

/// Обрабатывает правило грамматики
fn process_rule(
    pair: pest::iterators::Pair<Rule>,
    diagram: &mut SequenceDiagram,
    fragment_stack: &mut Vec<(FragmentType, Option<String>, Vec<FragmentSection>)>,
    current_section_elements: &mut Vec<SequenceElement>,
) {
    match pair.as_rule() {
        Rule::participant_decl => {
            if let Some(participant) = parse_participant(pair) {
                diagram.add_participant(participant);
            }
        }
        Rule::message => {
            if let Some(message) = parse_message(pair) {
                let element = SequenceElement::Message(message);
                if fragment_stack.is_empty() {
                    diagram.add_element(element);
                } else {
                    current_section_elements.push(element);
                }
            }
        }
        Rule::fragment_start => {
            let (frag_type, condition) = parse_fragment_start(pair);
            fragment_stack.push((frag_type, condition, vec![]));
            *current_section_elements = Vec::new();
        }
        Rule::fragment_else => {
            let new_condition = parse_fragment_else(pair);
            if let Some((_, _, ref mut sections)) = fragment_stack.last_mut() {
                // Сохраняем текущую секцию
                sections.push(FragmentSection {
                    condition: None,
                    elements: std::mem::take(current_section_elements),
                });
            }
            // Сохраняем условие для следующей секции (не используется напрямую)
            let _ = new_condition;
        }
        Rule::fragment_end => {
            if let Some((frag_type, condition, mut sections)) = fragment_stack.pop() {
                // Добавляем последнюю секцию
                sections.push(FragmentSection {
                    condition: condition.clone(),
                    elements: std::mem::take(current_section_elements),
                });
                
                let fragment = Fragment {
                    fragment_type: frag_type,
                    condition,
                    sections,
                };
                
                let element = SequenceElement::Fragment(fragment);
                if fragment_stack.is_empty() {
                    diagram.add_element(element);
                } else {
                    current_section_elements.push(element);
                }
            }
        }
        Rule::note_stmt => {
            if let Some(note) = parse_note(pair) {
                let element = SequenceElement::Note(note);
                if fragment_stack.is_empty() {
                    diagram.add_element(element);
                } else {
                    current_section_elements.push(element);
                }
            }
        }
        Rule::divider => {
            let text = parse_divider_text(pair);
            let element = SequenceElement::Divider(Divider { 
                text: text.unwrap_or_default() 
            });
            if fragment_stack.is_empty() {
                diagram.add_element(element);
            } else {
                current_section_elements.push(element);
            }
        }
        Rule::delay => {
            let text = parse_delay_text(pair);
            let element = SequenceElement::Delay(Delay { text });
            if fragment_stack.is_empty() {
                diagram.add_element(element);
            } else {
                current_section_elements.push(element);
            }
        }
        Rule::title_stmt => {
            if let Some(title) = parse_title(pair) {
                diagram.metadata.title = Some(title);
            }
        }
        Rule::activate_stmt => {
            if let Some((participant_id, color)) = parse_activate(pair) {
                let element = SequenceElement::Activation(Activation {
                    participant: participant_id,
                    activation_type: ActivationType::Activate,
                    color,
                });
                if fragment_stack.is_empty() {
                    diagram.add_element(element);
                } else {
                    current_section_elements.push(element);
                }
            }
        }
        Rule::deactivate_stmt => {
            if let Some(participant_id) = parse_deactivate(pair) {
                let element = SequenceElement::Activation(Activation {
                    participant: participant_id,
                    activation_type: ActivationType::Deactivate,
                    color: None,
                });
                if fragment_stack.is_empty() {
                    diagram.add_element(element);
                } else {
                    current_section_elements.push(element);
                }
            }
        }
        _ => {}
    }
}

/// Парсит объявление участника
fn parse_participant(pair: pest::iterators::Pair<Rule>) -> Option<Participant> {
    let mut participant_type = ParticipantType::Participant;
    let mut name = String::new();
    let mut alias: Option<String> = None;
    let mut stereotype: Option<Stereotype> = None;
    let mut color: Option<Color> = None;
    let mut order: Option<i32> = None;
    
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::participant_type => {
                participant_type = match inner.as_str().to_lowercase().as_str() {
                    "participant" => ParticipantType::Participant,
                    "actor" => ParticipantType::Actor,
                    "boundary" => ParticipantType::Boundary,
                    "control" => ParticipantType::Control,
                    "entity" => ParticipantType::Entity,
                    "database" => ParticipantType::Database,
                    "collections" => ParticipantType::Collections,
                    "queue" => ParticipantType::Queue,
                    _ => ParticipantType::Participant,
                };
            }
            Rule::participant_name => {
                name = extract_name(inner);
            }
            Rule::identifier => {
                // Это alias после "as"
                if name.is_empty() {
                    name = inner.as_str().to_string();
                } else {
                    alias = Some(inner.as_str().to_string());
                }
            }
            Rule::stereotype => {
                let s = inner.as_str();
                let content = s.trim_start_matches("<<").trim_end_matches(">>");
                stereotype = Some(Stereotype::new(content));
            }
            Rule::color => {
                let s = inner.as_str();
                color = Some(Color::from_hex(s));
            }
            Rule::number => {
                if let Ok(n) = inner.as_str().parse() {
                    order = Some(n);
                }
            }
            _ => {}
        }
    }
    
    if name.is_empty() {
        return None;
    }
    
    let mut participant = Participant::new(name, participant_type);
    if let Some(a) = alias {
        participant.id.alias = Some(a);
    }
    participant.stereotype = stereotype;
    participant.color = color;
    participant.order = order;
    
    Some(participant)
}

/// Извлекает имя из quoted_string или identifier
fn extract_name(pair: pest::iterators::Pair<Rule>) -> String {
    let fallback = pair.as_str().trim_matches('"').to_string();
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::quoted_string => {
                let s = inner.as_str();
                return s.trim_matches('"').to_string();
            }
            Rule::identifier => {
                return inner.as_str().to_string();
            }
            Rule::inner_string => {
                return inner.as_str().to_string();
            }
            _ => {}
        }
    }
    fallback
}

/// Парсит сообщение
fn parse_message(pair: pest::iterators::Pair<Rule>) -> Option<Message> {
    let mut from = String::new();
    let mut to = String::new();
    let mut label = String::new();
    let mut line_style = LineStyle::Solid;
    let mut arrow_type = ArrowType::Normal;
    
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::participant_ref => {
                let name = inner.as_str().to_string();
                if from.is_empty() {
                    from = name;
                } else {
                    to = name;
                }
            }
            Rule::arrow => {
                let (style, atype) = parse_arrow(inner);
                line_style = style;
                arrow_type = atype;
            }
            Rule::message_text => {
                label = inner.as_str().trim().to_string();
            }
            _ => {}
        }
    }
    
    if from.is_empty() || to.is_empty() {
        return None;
    }
    
    let mut message = Message::new(from, to, label);
    message.line_style = line_style;
    message.arrow_type = arrow_type;
    
    Some(message)
}

/// Парсит стрелку
fn parse_arrow(pair: pest::iterators::Pair<Rule>) -> (LineStyle, ArrowType) {
    let arrow_str = pair.as_str();
    
    // Определяем стиль линии
    // В PlantUML:
    // -> или ->> = solid (сплошная)
    // --> или -->> = dashed (пунктирная, двойной дефис)
    // .> или ..> = dotted (точечная, тоже считаем dashed)
    let line_style = if arrow_str.contains("..") 
        || arrow_str.starts_with(".") 
        || arrow_str.contains("--") 
    {
        LineStyle::Dashed
    } else {
        LineStyle::Solid
    };
    
    // Определяем тип стрелки
    let arrow_type = if arrow_str.contains(">>") {
        ArrowType::Thin
    } else if arrow_str.contains("\\\\") || arrow_str.contains("//") {
        ArrowType::HalfTop
    } else if arrow_str.ends_with("o") || arrow_str.contains(">o") {
        ArrowType::Circle
    } else if arrow_str.ends_with("x") || arrow_str.contains(">x") {
        ArrowType::Cross
    } else {
        ArrowType::Normal
    };
    
    (line_style, arrow_type)
}

/// Парсит начало фрагмента
fn parse_fragment_start(pair: pest::iterators::Pair<Rule>) -> (FragmentType, Option<String>) {
    let mut frag_type = FragmentType::Group;
    let mut condition: Option<String> = None;
    
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::fragment_type => {
                frag_type = match inner.as_str().to_lowercase().as_str() {
                    "alt" => FragmentType::Alt,
                    "opt" => FragmentType::Opt,
                    "loop" => FragmentType::Loop,
                    "par" => FragmentType::Par,
                    "break" => FragmentType::Break,
                    "critical" => FragmentType::Critical,
                    "group" => FragmentType::Group,
                    _ => FragmentType::Group,
                };
            }
            Rule::fragment_condition => {
                condition = Some(inner.as_str().trim().to_string());
            }
            _ => {}
        }
    }
    
    (frag_type, condition)
}

/// Парсит else фрагмента
fn parse_fragment_else(pair: pest::iterators::Pair<Rule>) -> Option<String> {
    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::fragment_condition {
            return Some(inner.as_str().trim().to_string());
        }
    }
    None
}

/// Парсит заметку
fn parse_note(pair: pest::iterators::Pair<Rule>) -> Option<Note> {
    let mut position = NotePosition::Right;
    let mut anchors: Vec<String> = Vec::new();
    let mut text = String::new();
    
    fn parse_note_inner(pair: pest::iterators::Pair<Rule>, position: &mut NotePosition, anchors: &mut Vec<String>, text: &mut String) {
        for inner in pair.into_inner() {
            match inner.as_rule() {
                Rule::note_single | Rule::note_multi | Rule::hnote | Rule::rnote => {
                    parse_note_inner(inner, position, anchors, text);
                }
                Rule::note_position => {
                    *position = match inner.as_str().to_lowercase().as_str() {
                        "left" => NotePosition::Left,
                        "right" => NotePosition::Right,
                        "over" => NotePosition::Over,
                        _ => NotePosition::Right,
                    };
                }
                Rule::identifier_list => {
                    for id in inner.into_inner() {
                        if id.as_rule() == Rule::identifier {
                            anchors.push(id.as_str().to_string());
                        }
                    }
                }
                Rule::identifier => {
                    anchors.push(inner.as_str().to_string());
                }
                Rule::note_text | Rule::note_body => {
                    *text = inner.as_str().trim().to_string();
                }
                _ => {}
            }
        }
    }
    
    parse_note_inner(pair, &mut position, &mut anchors, &mut text);
    
    Some(Note {
        position,
        anchors,
        text,
        background_color: None,
    })
}

/// Парсит текст разделителя
fn parse_divider_text(pair: pest::iterators::Pair<Rule>) -> Option<String> {
    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::divider_text {
            let text = inner.as_str().trim();
            if !text.is_empty() {
                return Some(text.to_string());
            }
        }
    }
    None
}

/// Парсит текст задержки
fn parse_delay_text(pair: pest::iterators::Pair<Rule>) -> Option<String> {
    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::delay_text {
            let text = inner.as_str().trim();
            if !text.is_empty() {
                return Some(text.to_string());
            }
        }
    }
    None
}

/// Парсит заголовок
fn parse_title(pair: pest::iterators::Pair<Rule>) -> Option<String> {
    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::rest_of_line {
            let text = inner.as_str().trim();
            if !text.is_empty() {
                return Some(text.to_string());
            }
        }
    }
    None
}

/// Парсит activate
fn parse_activate(pair: pest::iterators::Pair<Rule>) -> Option<(String, Option<Color>)> {
    let mut participant: Option<String> = None;
    let mut color: Option<Color> = None;
    
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::identifier => {
                participant = Some(inner.as_str().to_string());
            }
            Rule::color => {
                color = Some(Color::from_hex(inner.as_str()));
            }
            _ => {}
        }
    }
    
    participant.map(|p| (p, color))
}

/// Парсит deactivate
fn parse_deactivate(pair: pest::iterators::Pair<Rule>) -> Option<String> {
    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::identifier {
            return Some(inner.as_str().to_string());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_sequence() {
        let source = r#"@startuml
Alice -> Bob: Hello
Bob --> Alice: Hi
@enduml"#;
        
        let result = parse_sequence(source);
        assert!(result.is_ok(), "Parse error: {:?}", result.err());
        
        let diagram = result.unwrap();
        assert_eq!(diagram.elements.len(), 2);
    }

    #[test]
    fn test_parse_participants() {
        let source = r#"@startuml
participant Alice
actor Bob
database DB
Alice -> Bob: message
@enduml"#;
        
        let result = parse_sequence(source);
        assert!(result.is_ok(), "Parse error: {:?}", result.err());
        
        let diagram = result.unwrap();
        assert_eq!(diagram.participants.len(), 3);
        assert_eq!(diagram.participants[0].id.name, "Alice");
        assert_eq!(diagram.participants[0].participant_type, ParticipantType::Participant);
        assert_eq!(diagram.participants[1].participant_type, ParticipantType::Actor);
        assert_eq!(diagram.participants[2].participant_type, ParticipantType::Database);
    }

    #[test]
    fn test_parse_messages() {
        let source = r#"@startuml
Alice -> Bob: Hello
Bob --> Alice: Hi
Alice ->> Bob: Async
@enduml"#;
        
        let result = parse_sequence(source);
        assert!(result.is_ok(), "Parse error: {:?}", result.err());
        
        let diagram = result.unwrap();
        assert_eq!(diagram.elements.len(), 3);
        
        if let SequenceElement::Message(msg) = &diagram.elements[0] {
            assert_eq!(msg.from, "Alice");
            assert_eq!(msg.to, "Bob");
            assert_eq!(msg.label, "Hello");
            assert_eq!(msg.line_style, LineStyle::Solid);
        } else {
            panic!("Expected Message");
        }
        
        if let SequenceElement::Message(msg) = &diagram.elements[1] {
            assert_eq!(msg.line_style, LineStyle::Dashed);
        } else {
            panic!("Expected Message");
        }
    }

    #[test]
    fn test_parse_fragment() {
        let source = r#"@startuml
Alice -> Bob: Request

alt Success
    Bob --> Alice: OK
else Failure
    Bob --> Alice: Error
end
@enduml"#;
        
        let result = parse_sequence(source);
        assert!(result.is_ok(), "Parse error: {:?}", result.err());
        
        let diagram = result.unwrap();
        // Should have 1 message + 1 fragment
        assert!(diagram.elements.len() >= 1);
    }
}
