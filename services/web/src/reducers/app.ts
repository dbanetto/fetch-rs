export interface IAppState {
  errorMessage: string;
  showError: boolean;
}

const INITAL_SATE: IAppState = {
  errorMessage: "",
  showError: false,
};

export const appReducer = (state: IAppState = INITAL_SATE, action): IAppState => {
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
