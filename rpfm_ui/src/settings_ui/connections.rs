//---------------------------------------------------------------------------//
// Copyright (c) 2017-2020 Ismael Gutiérrez González. All rights reserved.
//
// This file is part of the Rusted PackFile Manager (RPFM) project,
// which can be found here: https://github.com/Frodo45127/rpfm.
//
// This file is licensed under the MIT license, which can be found here:
// https://github.com/Frodo45127/rpfm/blob/master/LICENSE.
//---------------------------------------------------------------------------//

/*!
Module with all the code to connect `SettingsUI` signals with their corresponding slots.

This module is, and should stay, private, as it's only glue between the `SettingsUI` and `SettingsUISlots` structs.
!*/

use super::{SettingsUI, slots::SettingsUISlots};

/// This function connects all the actions from the provided `SettingsUI` with their slots in `SettingsUIlots`.
///
/// This function is just glue to trigger after initializing both, the actions and the slots. It's here
/// to not polute the other modules with a ton of connections.
pub unsafe fn set_connections(settings_ui: &SettingsUI, slots: &SettingsUISlots) {
    settings_ui.paths_mymod_button.released().connect(&slots.select_mymod_path);
    settings_ui.paths_zip_button.released().connect(&slots.select_zip_path);

    for (key, button) in settings_ui.paths_games_buttons.iter() {
        button.released().connect(&slots.select_game_paths[key]);
    }

    settings_ui.debug_clear_autosave_folder_button.released().connect(&slots.clear_autosaves);
    settings_ui.debug_clear_schema_folder_button.released().connect(&slots.clear_schemas);

    settings_ui.button_box_shortcuts_button.released().connect(&slots.shortcuts);
    settings_ui.button_box_restore_default_button.released().connect(&slots.restore_default);
    settings_ui.button_box_text_editor_settings_button.released().connect(&slots.text_editor);
    settings_ui.button_box_font_settings_button.released().connect(&slots.font_settings);
    settings_ui.button_box_accept_button.released().connect(settings_ui.dialog.slot_accept());
    settings_ui.button_box_cancel_button.released().connect(settings_ui.dialog.slot_close());
}
