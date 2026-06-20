export enum TaskStatus {
  PENDING = "pending",
  DONE = "done",
}

export interface ToDoItem {
  title: string;
  status: TaskStatus;
}

export interface ToDoItems {
  pending: ToDoItem[];
  done: ToDoItem[];
}
