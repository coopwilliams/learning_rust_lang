// example client.

// The Quiz in chapter 17 of the Rust book
// Says that, if we're designing for this
// particular use case (lines 11-23),
// we should stick with the trait `Widget`.
// It also seems to say that only this definition
// of the trait will satisfy the requirement.

// But here I show that 2nd claim  isn't quite true.
// Widget2 thru Widget5 also satisfy it,
// albeit in ways that are wholly unnecessary.
// I had to assure myself I wasn't wrong.

use trait_trees::{Widget, Widget2, Widget3, Widget4, Widget5};

fn main() {
    enum WidgetImpl {
        Button {},
        Text {},
    }

    impl Widget for WidgetImpl {
        fn render(&self) -> Vec<Self> {
            vec![
                WidgetImpl::Text {}, 
                WidgetImpl::Button {}
            ]
        }
    }
    
    impl Widget2<WidgetImpl> for WidgetImpl {
        fn render(&self) -> Vec<WidgetImpl> {
            vec![
                WidgetImpl::Text {}, 
                WidgetImpl::Button {}
            ]
        }
    }

    impl Widget3 for WidgetImpl {
        type Children = WidgetImpl;
        fn render(&self) -> Vec<Self::Children> {
            vec![
                WidgetImpl::Text {}, 
                WidgetImpl::Button {}
            ]
        }
    }

    impl Widget4 for WidgetImpl {
        fn render(&self) -> Vec<&dyn Widget4> {
            vec![
                &WidgetImpl::Text {}, 
                &WidgetImpl::Button {}
            ]
        }
    }

    impl Widget5 for WidgetImpl {
        fn render(&self) -> Vec<Box<dyn Widget5>> {
            vec![
                Box::new(WidgetImpl::Text {}), 
                Box::new(WidgetImpl::Button {})
            ]
        }
    }
}
