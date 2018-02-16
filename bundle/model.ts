interface Series {
    id: number;
    title: string;
    poster_url: string;
}

interface InfoBlob {
    id: number,
    series_id: number,
    blob: object,
    primary: boolean,
    info_type: string,
}
