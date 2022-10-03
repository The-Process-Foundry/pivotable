//! A simple button that can be used for submit/clear
//!
//! TODO: Move this to yewzy

use crate::local::*;
use std::{
  collections::{hash_map::RandomState, HashMap, HashSet},
  iter::FromIterator,
};

#[derive(Clone, Debug)]
pub enum ButtonMsg {}

#[macro_export]
macro_rules! classmap(
  ($($val:expr$(,)?)+) => {
    ClassMap::new(vec![$($val.to_string()),+])
  }
);

/// States that can be passed in a prop and used to toggle the display
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ButtonState {
  /// Remove the button so it takes up no space and isn't in the tab order
  Hidden,

  /// The default display
  Active,

  /// An unclickable, "greyed" out display
  Inactive,

  /// How to alter the display after the button has been clicked once and is waiting for a result
  Pending,
}

impl Default for ButtonState {
  fn default() -> Self {
    Self::Active
  }
}

/// A more complicated method of building classes, allowing toggles and dynamic rules
///
/// This is based on React classnames: https://www.npmjs.com/package/classnames
/// TODO: Make this an independent crate
#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct ClassMap {
  /// Classes to toggle when the pseudoclass is active
  ///
  /// The empty string is the base classes.
  classes: HashMap<String, HashSet<String>>,
}

impl ClassMap {
  /// Creates a new classmap for the default
  pub fn new(classes: Vec<String>) -> ClassMap {
    let mut map = HashMap::new();
    map.insert("".to_string(), HashSet::from_iter(classes));
    ClassMap { classes: map }
  }
}

impl From<ClassMap> for Classes {
  fn from(map: ClassMap) -> Classes {
    let mut classes = Classes::new();
    let _ = map.classes.get("").map(|base| {
      for cls in base.iter() {
        classes.push(cls.clone())
      }
    });

    for (key, value) in map.classes.iter() {
      if key.is_empty() {
        continue;
      };
      for cls in value.iter() {
        classes.push(format!("{}:{}", key, cls))
      }
    }

    classes
  }
}

/// Configuration styles that can be toggled via state
#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct ButtonStateConfig {
  /// Classes to be applied for the given state
  states: HashMap<ButtonState, ClassMap>,
}

impl ButtonStateConfig {
  pub fn get_classes(&self, state: ButtonState) -> Classes {
    let conf = self
      .states
      .get(&state)
      .or_else(|| self.states.get(&ButtonState::default()))
      .unwrap();
    Classes::from(conf.clone())
  }
}

impl Default for ButtonStateConfig {
  fn default() -> Self {
    let mut states = HashMap::new();
    states.insert(ButtonState::Hidden, classmap!("invisible", "hidden"));
    states.insert(
      ButtonState::Active,
      classmap!(
        "inline-block",
        "h-full",
        "w-full",
        "bg-gradient-to-b",
        "from-[#FF7E31]",
        "to-[#E62C03]",
        "rounded-md",
        "overflow-hidden",
        "shadow-lg",
        "shadow-neutral-600",
        "transition-shadow",
      ),
    );
    Self { states }
  }
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct ButtonProps {
  /// Unique Identifier for Button
  #[prop_or_default]
  pub id: String,

  /// CSS Classes used for rendering
  #[prop_or_default]
  pub class: Classes,

  /// Child components
  #[prop_or_default]
  pub children: Children,

  /// Action to be taken when the button is clicked
  pub onclick: Callback<MouseEvent>,

  /// Trigger various display states
  #[prop_or_default]
  pub config: ButtonStateConfig,

  #[prop_or_default]
  pub state: ButtonState,
}

/// Button
#[derive(Debug, Clone)]
pub struct Button {
  config: ButtonStateConfig,
}

impl Button {}

impl Component for Button {
  type Message = ButtonMsg;
  type Properties = ButtonProps;

  fn create(_ctx: &Context<Button>) -> Self {
    debug!("Creating the Button");

    Button {
      config: ButtonStateConfig::default(),
    }
  }

  fn view(&self, ctx: &Context<Button>) -> Html {
    debug!("Rendering view for Button");
    let class = self.config.get_classes(ctx.props().state.clone());
    debug!("Button Classes: {:?}", class);
    let onclick = ctx.props().onclick.clone();

    html! {
      <div class={classes!("h-full", "w-full")}>
        <button {class} {onclick}>
          <p class={classes!("text-white", "text-center", "leading-10", "mx-4")}>
            {for ctx.props().children.iter()}
          </p>
        </button>
      </div>
    }
  }
}
