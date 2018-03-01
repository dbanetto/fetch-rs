interface Series {
    id: number;
    title: string;
    poster_url: string;
}

interface InfoBlob {
    id: number,
    series_id: number,
    blob: any,
    info_type: string,
}

interface HandlerProps {
    blob: any;
    edit: boolean;
    handleUpdate: any;
    name: string;
}

type SeriesFull = Series & {info: Array<InfoBlob>; };
