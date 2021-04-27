export class UnknownItemTypeError extends Error {
  constructor(private readonly itemType: string) {
    super();
  }

  public toString() {
    return `Unknown item type: ${this.itemType}`;
  }
}
