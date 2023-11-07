export namespace DashbookParser {
  export function parse(input: string): Cell[];
  export function generate(cells: Cell[]): string;
}
export type CellType = CellTypeMarkdown | CellTypeCode | CellTypeQuery;
export interface CellTypeMarkdown {
  tag: 'markdown',
}
export interface CellTypeCode {
  tag: 'code',
}
export interface CellTypeQuery {
  tag: 'query',
}
export type CellOutput = CellOutputText | CellOutputHtml;
export interface CellOutputText {
  tag: 'text',
  val: string,
}
export interface CellOutputHtml {
  tag: 'html',
  val: string,
}
export interface Cell {
  cellType: CellType,
  size: number,
  source: string,
  outputs: CellOutput[],
}
export type Error = ErrorParseError;
export interface ErrorParseError {
  tag: 'parse-error',
}
