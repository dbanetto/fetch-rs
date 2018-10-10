import "../model";

export const getStatus = () => ({
  type: "FETCH_GET_STATUS",
});

export const finishedStatus = (status) => ({
  status,
  type: "FETCH_FINISH_STATUS",
});

export const callFetch = () => ({
  type: "FETCH_CALL",
});

export const finishedCallFetch = (results) => ({
  results,
  type: "FETCH_CALL_FINISHED",
});
