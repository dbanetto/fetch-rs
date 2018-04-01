
const INITAL_SATE = {
  message: "",
  showError: false,
};

export const appReducer = (state, action) => {
  if (!state) {
    return INITAL_SATE;
  }

  switch (action.type) {
    case "SHOW_ERROR":
      return { ...state, showError: true, errorMessage: action.message };
    case "HIDE_ERROR":
      return { ...state, showError: false, errorMessage: "" };
    default:
      return state;
  }
};

export default appReducer;
