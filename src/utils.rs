use crate::Result;
use roxmltree::{Document, Node};

#[doc(hidden)]
#[macro_export]
macro_rules! args {
    ( $( $var:literal: $e:expr ),* ) => { &{
        let mut s = String::new();
        $(
            s.push_str(concat!("<", $var, ">"));
            s.push_str(&$e.to_string());
            s.push_str(concat!("</", $var, ">"));
        )*
        s
    } }
}

pub trait HashMapExt {
    fn extract(&mut self, key: &str) -> Result<String>;
}
impl HashMapExt for std::collections::HashMap<String, String> {
    fn extract(&mut self, key: &str) -> Result<String> {
        self.remove(key).ok_or_else(|| {
            upnp::Error::XMLMissingElement("UPnP Response".to_string(), key.to_string())
        })
    }
}

pub fn duration_to_str(duration: &std::time::Duration) -> String {
    let seconds_total = duration.as_secs();
    let seconds = seconds_total % 60;
    let minutes = (seconds_total / 60) % 60;
    let hours = seconds_total / 3600;

    return format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
}
pub fn duration_from_str(s: &str) -> Result<std::time::Duration> {
    let opt = (|| {
        let mut split = s.splitn(3, ':');
        let hours = split.next()?.parse::<u64>().ok()?;
        let minutes = split.next()?.parse::<u64>().ok()?;
        let seconds = split.next()?.parse::<u64>().ok()?;

        Some(std::time::Duration::from_secs(
            hours * 3600 + minutes * 60 + seconds,
        ))
    })();

    opt.ok_or(upnp::Error::ParseError("invalid duration"))
}

pub fn parse_bool(s: String) -> Result<bool> {
    s.parse().map_err(upnp::Error::invalid_response)
}

pub fn parse_node_text(node: Node) -> Result<String> {
    node.text()
        .ok_or_else(|| upnp::Error::XMLMissingText(node.tag_name().name().to_string()))
        .map(|x| x.to_string())
}

pub fn find_root_node<'a, 'input: 'a>(
    document: &'input Document,
    element: &str,
    docname: &str,
) -> Result<Node<'a, 'input>> {
    document
        .descendants()
        .filter(roxmltree::Node::is_element)
        .find(|n| n.tag_name().name().eq_ignore_ascii_case(element))
        .ok_or_else(|| upnp::Error::XMLMissingElement(docname.to_string(), element.to_string()))
}
