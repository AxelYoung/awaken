#[macro_export]
macro_rules! iterate_entities {
  // This matches a single immutable component
  ($world:expr, [$components:ident], $function:expr) => {{
    'macro_logic: {
      let component_vec = match $world.borrow_components::<$components>() {
        Some(vec) => vec,
        None => break 'macro_logic
      };
      let $components = component_vec.iter();
      
      let filtered_iter = $components.filter_map(|($components)| Some((($components.as_ref()?))));
  
      for ($components) in filtered_iter {
        $function($components);
      }
    }
  }};

  // This matches any number of immutable components greater than one
  ($world:expr, [$($components:ident),*], $function:expr) => {{
    'macro_logic: {
      $(
        let component_vec = match $world.borrow_components::<$components>() {
          Some(vec) => vec,
          None => break 'macro_logic
        };
        let $components = component_vec.iter();
      )*
  
      let iter = itertools::multizip(($($components),*));
      let filtered_iter = iter.filter_map(|($($components),*)| Some((($($components.as_ref()?),*))));
  
      for ($($components),*) in filtered_iter {
        $function($($components),*);
      }
    }
  }};

  // This matches a single mutable component
  ($world:expr, ($mut_components:ident), $function:expr) => {{
    'macro_logic: {
      let mut mut_components_vec = match $world.borrow_components_mut::<$mut_components>() {
        Some(vec) => vec,
        None => break 'macro_logic
      };
      let $mut_components = mut_components_vec.iter_mut();
  
      let iter = $mut_components.filter_map(|($mut_components)| Some((($mut_components.as_mut()?))));
  
      for ($mut_components) in iter {
        $function($mut_components);
      }
    }
  }};

  // This matches any number of mutable components greater than one
  ($world:expr, ($($mut_components:ident),*), $function:expr) => {{
    'macro_logic: {
      $(
        let mut mut_components_vec = match $world.borrow_components_mut::<$mut_components>() {
          Some(vec) => vec,
          None => break 'macro_logic
        };
        let $mut_components = mut_components_vec.iter_mut();
      )*

      let iter = itertools::multizip(($($mut_components),*));
      let iter = iter.filter_map(|($($mut_components),*)| Some((($($mut_components.as_mut()?),*))));

      for ($($mut_components),*) in iter {
        $function($($mut_components),*);
      }
    }
  }};
  
  // This matches any number of immutable and mutable components
  ($world:expr, [$($components:ident),*], ($($mut_components:ident),*), $function:expr) => {{
    'macro_logic : {
      $(
        let component_vec = match $world.borrow_components::<$components>() {
          Some(vec) => vec,
          None => break 'macro_logic
        };
        let $components = component_vec.iter();
      )*
  
      $(
        let mut mut_components_vec = match $world.borrow_components_mut::<$mut_components>() {
          Some(vec) => vec,
          None => break 'macro_logic
        };
        let $mut_components = mut_components_vec.iter_mut();
      )*
  
      let iter = itertools::multizip(($($components),*, $($mut_components),*));
      let iter = iter.filter_map(|($($components),*, $($mut_components),*)| Some((($($components.as_ref()?),*, $($mut_components.as_mut()?),*))));
  
      for ($($components),*, $($mut_components),*) in iter {
        $function($($components),*, $($mut_components),*);
      }
    }
  }};
}

#[macro_export]
macro_rules! iterate_entities_with_id {
  // This matches a single immutable component
  ($world:expr, [$components:ident], $function:expr) => {{
    'macro_logic: {
      let component_vec = match $world.borrow_components::<$components>() {
        Some(vec) => vec,
        None => break 'macro_logic
      };
      let $components = component_vec.iter();
      
      let filtered_iter = $components.enumerate().filter_map(|(id, $components)| Some(((id, $components.as_ref()?))));
  
      for (id, $components) in filtered_iter {
        $function(id, $components);
      }
    }
  }};
  
  // This matches any number of immutable components
  ($world:expr, [$($components:ident),*], $function:expr) => {{
    'macro_logic : {
      $(
        let component_vec = match $world.borrow_components::<$components>() {
          Some(vec) => vec,
          None => break 'macro_logic
        };
        let $components = component_vec.iter();
      )*
  
      let iter = itertools::multizip(($($components),*));
      let filtered_iter = iter.enumerate().filter_map(|(id, ($($components),*))| Some(((id, $($components.as_ref()?),*))));
  
      for (id, $($components),*) in filtered_iter {
        $function(id, $($components),*);
      }
    }
  }};

  // This matches a single mutable component
  ($world:expr, ($mut_components:ident), $function:expr) => {{
    let mut mut_components_vec = $world.borrow_components_mut::<$mut_components>().unwrap();
    let $mut_components = mut_components_vec.iter_mut();

    let iter = $mut_components.enumerate().filter_map(|(id, $mut_components)| Some(((id, $mut_components.as_mut()?))));

    for (id, $mut_components) in iter {
      $function($mut_components);
    }
  }};

  // This matches any number of mutable components greater than one
  ($world:expr, ($($mut_components:ident),*), $function:expr) => {{
    $(
      let mut mut_components_vec = match $world.borrow_components_mut::<$mut_components>() {
        Some(vec) => vec,
        None => return
      };
      let $mut_components = mut_components_vec.iter_mut();
    )*

    let iter = itertools::multizip(($($mut_components),*));
    let iter = iter.enumerate().filter_map(|(id, ($($mut_components),*))| Some(((id, $($mut_components.as_mut()?),*))));

    for (id, $($mut_components),*) in iter {
      $function(id, $($mut_components),*);
    }
  }};
  
  // This matches any number of immutable and mutable components
  ($world:expr, [$($components:ident),*], ($($mut_components:ident),*), $function:expr) => {{
    'macro_logic: {
      $(
        let component_vec = 
          match $world.borrow_components::<$components>() {
            Some(vec) => vec,
            None => break 'macro_logic
        };
        let $components = component_vec.iter();
      )*
  
      $(
        let mut mut_components_vec = 
          match $world.borrow_components_mut::<$mut_components>() {
            Some(vec) => vec,
            None => break 'macro_logic
        };

        let $mut_components = mut_components_vec.iter_mut();
      )*
  
      let iter = itertools::multizip(
        ($($components),*, 
        $($mut_components),*)
      )
      ;
      let iter = iter.enumerate()
        .filter_map(
          |(id, ($($components),*, $($mut_components),*))|
          Some(
            ((id, $($components.as_ref()?),*,
             $($mut_components.as_mut()?),*))
          )
      );
  
      for (id, $($components),*, $($mut_components),*) in iter {
        $function(id, $($components),*, $($mut_components),*);
      }
    }
  }};
}