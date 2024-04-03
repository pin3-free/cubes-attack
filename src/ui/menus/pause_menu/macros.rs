#[macro_export]
macro_rules! column {
   ( $($i:ident: $prop:expr),* $(; $($st:ident: $stprop:expr),* )?  ) => {
        {
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    $(
                        $i: $prop,
                    )*
                    ..default()
                },
                $($(
                    $st: $stprop,
                )*)?
                ..default()
            };
        }
    };
}
pub use column;

#[macro_export]
macro_rules! text {
    (  $($i:ident: $prop:expr,)* $text:expr ) => {
            {
                TextBundle {
                    text: Text::from_section(
                        $text,
                        TextStyle {
                            $($i: $prop,)*
                            ..default()
                        }
                    ),
                    ..default()
                }
        }
    };
}
pub use text;
