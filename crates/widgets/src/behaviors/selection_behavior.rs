use std::cell::Cell;

use crate::prelude::*;

/// The `SelectionBehaviorState` handles the `SelectionBehavior` widget.
#[derive(Default)]
pub struct SelectionBehaviorState {
    selected: Cell<bool>,
}

impl SelectionBehaviorState {
    fn toggle_selection(&self) {
        self.selected.set(!self.selected.get());
    }
}

impl State for SelectionBehaviorState {
    fn update(&self, context: &mut Context<'_>) {
        if context.widget().get::<Selected>().0 == self.selected.get() {
            return;
        }

        context.widget().set(Selected(self.selected.get()));

        if self.selected.get() {
            add_selector_to_widget("selected", &mut context.widget());
        } else {
            remove_selector_from_widget("selected", &mut context.widget());
        }

        let element = context.widget().clone::<Selector>().0.element.unwrap();

        if let Some(parent) = context.parent_entity_by_element(element) {
            context.update_theme_properties(parent);
        }
    }
}

widget!(
    /// The `SelectionBehavior` widget is used to handle internal the pressed behavior of a widget.
    /// 
    /// **CSS element:** `check-box`
    SelectionBehavior<SelectionBehaviorState>: MouseHandler {
        /// Sets or shares the css selector property. 
        selector: Selector,

        /// Sets or shares the selected property. 
        selected: Selected
    }
);

impl Template for SelectionBehavior {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        let state = self.clone_state();

        self.name("SelectionBehavior")
            .selector("")
            .selected(true)
            .on_click(move |_| {
                state.toggle_selection();
                false
            })
    }
}
