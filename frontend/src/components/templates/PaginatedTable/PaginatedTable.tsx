import React from "react";
import { Icon, Menu, Table } from 'semantic-ui-react'
import {Identifiable} from "../../../typings/commonTypes";

export interface ColumnDefinition<T> {
    name: string,
    cell: React.FC<T>,
}

export interface LeaderboardTableProps<T extends Identifiable> {
    columns: ColumnDefinition<T>[],
    rows: T[],
    page: number,
    pageCount: number,
    onSetPage?: (newPage: number) => void;
}

export const PaginatedTable = <T extends Identifiable,>(props: React.PropsWithChildren<LeaderboardTableProps<T>>) => {
    const { columns, rows, page, pageCount, onSetPage } = props;
    const setPage = (newPage: number) => {
        if (onSetPage && newPage >= 0 && newPage < pageCount) {
            onSetPage(newPage);
        }
    };

    return (
        <Table celled>
            <Table.Header>
                <Table.Row>
                    {columns.map((col) => <Table.HeaderCell key={col.name} >{col.name}</Table.HeaderCell>)}
                </Table.Row>
            </Table.Header>

            <Table.Body>
                {rows.map((row) =>
                    <Table.Row key={row.id}>
                        {columns.map((col) => <Table.Cell key={col.name}>{col.cell(row)}</Table.Cell>)}
                    </Table.Row>
                )}
            </Table.Body>
            <Table.Footer>
                <Table.Row>
                    <Table.HeaderCell colSpan={columns.length+''}>
                        <Menu floated='right' pagination>
                            <Menu.Item onClick={() => setPage(page-1)} as='a' icon>
                                <Icon name='chevron left' />
                            </Menu.Item>
                            {page !== 0 && <Menu.Item onClick={() => setPage(page-1)} as='a'>{page-1}</Menu.Item>}
                            <Menu.Item as='a'>{page}</Menu.Item>
                            {page <= pageCount-1 && <Menu.Item onClick={() => setPage(page+1)} as='a'>{page+1}</Menu.Item>}
                            <Menu.Item onClick={() => setPage(page+1)} as='a' icon>
                                <Icon name='chevron right' />
                            </Menu.Item>
                        </Menu>
                    </Table.HeaderCell>
                </Table.Row>
            </Table.Footer>
        </Table>
    );
};
