interface ISeries {
    id: number;
    title: string;
    poster_url: string;
}

interface IInfoBlob {
    id: number;
    series_id: number;
    blob: any;
    info_type: string;
}

interface IHandlerProps {
    blob: any;
    edit: boolean;
    handleUpdate: any;
    name: string;
}

type SeriesFull = ISeries & {info: IInfoBlob[]; };
