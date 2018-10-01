export const showError = (message: string) => ({
  message,
  type: "SHOW_ERROR",
});

export const hideError = () => ({
  type: "HIDE_ERROR",
});
