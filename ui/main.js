document.addEventListener('DOMContentLoaded', () => {
  const inputElement = document.getElementById('command-input');

  // Tauri window listeners
  const appWindow = getCurrentWindow();

  appWindow.listen('tauri://focus', () => {
    inputElement.focus();
  });

  appWindow.listen('window-shown', () => {
    inputElement.value = '';
    inputElement.focus();
  });

  inputElement.addEventListener('keydown', async (e) => {
    if (e.key === 'Enter') {
      const command = inputElement.value.trim();
      if (command) {
        inputElement.value = '';
        // Hide window immediately after sending command
        await appWindow.hide();
        // Invoke rust command
        await invoke('process_command', { command });
      }
    } else if (e.key === 'Escape') {
      inputElement.value = '';
      await appWindow.hide();
    }
  });

  // Ensure window is hidden initially until shown by hotkey
  appWindow.hide();
});
