import "../model";

export const getStatus = () => ({
  type: "FETCH_GET_STATUS",
});

export const finishedStatus = (status) => ({
  status,
  type: "FETCH_FINISH_STATUS",
});
