use failure::{format_err, Error};
use roxmltree::Node;
use try_from::{TryFrom, TryInto};

pub mod assertion;
pub mod encryption;
pub mod response;
pub mod signature;

pub(crate) fn try_child<'a, 'd: 'a>(
    node: Node<'a, 'd>,
    element_name: &str,
) -> Result<Node<'a, 'd>, Error> {
    node.children()
        .find(|c| c.tag_name().name() == element_name)
        .ok_or_else(|| {
            format_err!(
                "{} element not found within {}",
                element_name,
                node.tag_name().name(),
            )
        })
}

pub(crate) fn maybe_child<'a, 'd: 'a, T>(
    node: Node<'a, 'd>,
    element_name: &str,
) -> Result<Option<T>, Error>
where
    T: TryFrom<Node<'a, 'd>, Err = Error>,
{
    node.children()
        .find(|c| c.tag_name().name() == element_name)
        .map(|c| c.try_into())
        .transpose()
}

pub(crate) fn try_attribute<'a, 'd: 'a>(
    node: Node<'a, 'd>,
    attribute_name: &str,
) -> Result<String, Error> {
    node.attribute(attribute_name)
        .map(|a| a.into())
        .ok_or_else(|| {
            format_err!(
                "{} attribute not found within {}",
                attribute_name,
                node.tag_name().name(),
            )
        })
}
