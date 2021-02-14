use crate::*;

/// Adds selected units to SelectedUnits resource and removes dead entities
pub fn unit_selection_system(
    mouse_events: &Vec<MouseEvent>,
    selected_units: &mut SelectedUnits,
) -> SystemResult {
    for ev in mouse_events.iter() {
        if let MouseEvent::UnitSelected(e) = ev {
            // Don't select the unit twice:
            if !selected_units.units.iter().any(|&x| x == *e) {
                selected_units.units.push(e.clone());
            }
        }
    }
    Ok(())
}
