import * as React from "react";
import { render } from "react-dom";
import { connect } from "react-redux";

import "../model";
import handler from "./handler";
import InfoElement from "./infoElement";

interface IInfoListProps {
    value: IInfoBlob[];
    handleUpdate: (key: string, value: IInfoBlob[]) => void;
}

export default class InfoList extends React.PureComponent<IInfoListProps> {

    constructor(props) {
        super(props);

        this.handleAdd = this.handleAdd.bind(this);
    }

    public render() {
        return  (
            <div>
                <div className="info-list">
                    {this.props.value.map((ele, index) => this.buildElement(ele, index))}
                </div>

                <div className="select">
                    <select  id="type-selector">
                        {this.getTypes().map((t, i) => <option key={i.toString()} value={t.type}>{t.name}</option>)}
                    </select>
                </div>
                <span>&nbsp;</span>
                <button className="button" type="button" onClick={this.handleAdd}>Add</button>
            </div>
        );
    }

    private handleAdd() {
        const elements = this.props.value;
        const selection = document.getElementById("type-selector") as HTMLInputElement;
        const built = {
            blob: {},
            id: null,
            info_type: selection.value.toString(),
            series_id: null,
        };

        elements.push(built);

        this.props.handleUpdate("info", elements);
    }

    private handleDelete(key) {
        const elements = this.props.value;

        elements.splice(key, 1);

        this.props.handleUpdate("info", elements);
    }

    private handleUpdate(index: number, value: IInfoBlob) {
        const blobs = this.props.value;

        blobs[index] = value;

        this.props.handleUpdate("info", blobs);
    }

    private buildElement(ele, index) {
        return (<div key={index} className="info-list-item">
            <InfoElement
                handleDelete={this.handleDelete.bind(this, index)}
                handleUpdate={this.handleUpdate.bind(this, index)}
                value={ele}
            />
        </div>
        );
    }

    private getTypes() {
        return handler.listTypes();
    }

}
