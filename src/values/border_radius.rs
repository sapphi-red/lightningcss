use super::length::*;
use cssparser::*;
use super::traits::{Parse, ToCss};
use crate::properties::Property;
use super::rect::Rect;

#[derive(Debug, Clone, PartialEq)]
pub struct BorderRadius {
  top_left: Size2D,
  top_right: Size2D,
  bottom_left: Size2D,
  bottom_right: Size2D
}

impl Parse for BorderRadius {
  fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ()>> {
    let widths: Rect<LengthPercentage> = Rect::parse(input)?;
    let heights = if input.try_parse(|input| input.expect_delim('/')).is_ok() {
      Rect::parse(input)?
    } else {
      widths.clone()
    };

    Ok(BorderRadius {
      top_left: Size2D::new(widths.0, heights.0),
      top_right: Size2D::new(widths.1, heights.1),
      bottom_left: Size2D::new(widths.2, heights.2),
      bottom_right: Size2D::new(widths.3, heights.3)
    })
  }
}

impl ToCss for BorderRadius {
  fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result where W: std::fmt::Write {
    let widths = Rect::new(&self.top_left.width, &self.top_right.width, &self.bottom_left.width, &self.bottom_right.width);
    let heights = Rect::new(&self.top_left.height, &self.top_right.height, &self.bottom_left.height, &self.bottom_right.height);

    widths.to_css(dest)?;
    if widths != heights {
      dest.write_str(" / ")?;
      heights.to_css(dest)?;
    }

    Ok(())
  }
}

#[derive(Default, Debug)]
pub struct BorderRadiusHandler {
  top_left: Option<Size2D>,
  top_right: Option<Size2D>,
  bottom_left: Option<Size2D>,
  bottom_right: Option<Size2D>,
  decls: Vec<Property>
}

impl BorderRadiusHandler {
  pub fn handle_property(&mut self, property: &Property) -> bool {
    use Property::*;
    match property {
      BorderTopLeftRadius(val) => self.top_left = Some(val.clone()),
      BorderTopRightRadius(val) => self.top_right = Some(val.clone()),
      BorderBottomLeftRadius(val) => self.bottom_left = Some(val.clone()),
      BorderBottomRightRadius(val) => self.bottom_right = Some(val.clone()),
      BorderStartStartRadius(_) | BorderStartEndRadius(_) | BorderEndStartRadius(_) | BorderEndEndRadius(_) => {
        self.flush();
        self.decls.push(property.clone());
      }
      BorderRadius(val) => {
        self.decls.clear();
        self.top_left = Some(val.top_left.clone());
        self.top_right = Some(val.top_right.clone());
        self.bottom_left = Some(val.bottom_left.clone());
        self.bottom_right = Some(val.bottom_right.clone());
      }
      _ => return false
    }

    true
  }

  pub fn flush(&mut self) {
    let top_left = std::mem::take(&mut self.top_left);
    let top_right = std::mem::take(&mut self.top_right);
    let bottom_left = std::mem::take(&mut self.bottom_left);
    let bottom_right = std::mem::take(&mut self.bottom_right);

    if top_left.is_some() && top_right.is_some() && bottom_left.is_some() && bottom_right.is_some() {
      self.decls.push(Property::BorderRadius(BorderRadius {
        top_left: top_left.unwrap(),
        top_right: top_right.unwrap(),
        bottom_left: bottom_left.unwrap(),
        bottom_right: bottom_right.unwrap(),
      }))
    } else {
      if let Some(val) = top_left {
        self.decls.push(Property::BorderTopLeftRadius(val))
      }

      if let Some(val) = top_right {
        self.decls.push(Property::BorderTopRightRadius(val))
      }

      if let Some(val) = bottom_left {
        self.decls.push(Property::BorderBottomLeftRadius(val))
      }

      if let Some(val) = bottom_right {
        self.decls.push(Property::BorderBottomRightRadius(val))
      }
    }
  }

  pub fn finalize(&mut self) -> Vec<Property> {
    self.flush();
    std::mem::take(&mut self.decls)
  }
}
