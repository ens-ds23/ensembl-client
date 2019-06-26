import { ZmenuAction } from './zmenu-types';

class MockEventEmitter {
  private id: number = 0;

  private generateId() {
    return this.id + 1;
  }

  private updateId(id: number) {
    this.id = id;
  }

  public start(element: HTMLDivElement) {
    element.addEventListener('click', (event: MouseEvent) => {
      this.sendEvent(event, element);
    });
  }

  private sendEvent(event: MouseEvent, element: HTMLDivElement) {
    this.sendCloseEvent(element);
    this.sendShowEvent(event, element);
  }

  private sendCloseEvent(element: HTMLDivElement) {
    const event = new CustomEvent('bpane-zmenu', {
      detail: {
        id: this.id,
        action: ZmenuAction.DESTROY
      }
    });

    element.dispatchEvent(event);
  }

  private sendShowEvent(event: MouseEvent, element: HTMLDivElement) {
    const newId = this.generateId();
    const { left, top } = element.getBoundingClientRect();
    const coordinates = {
      x: event.clientX - left,
      y: event.clientY - top
    };

    const customEvent = new CustomEvent('bpane-zmenu', {
      detail: {
        id: newId,
        action: ZmenuAction.CREATE,
        anchor_coordinates: coordinates,
        content: [
          {
            id: 'gcid',
            lines: [
              [[{ markup: [], text: '%GC' }]],
              [
                [
                  { markup: ['strong'], text: 'percentage' },
                  { markup: [], text: ' GC' }
                ]
              ]
            ]
          }
        ]
      }
    });

    element.dispatchEvent(customEvent);

    this.updateId(newId);
  }
}

export default new MockEventEmitter();
