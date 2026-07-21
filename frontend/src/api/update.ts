import { ToDoItem, ToDoItems, TaskStatus } from "../interfaces/toDoItems";
import { patchCall } from "./utils";
import { Url } from "./url";

export async function updateToDoItemCall(name: string, status: TaskStatus, id: number) {
  const toDoItem: ToDoItem = {
    id: id,
    title: name,
    status: status,
  };
  return patchCall<ToDoItem, ToDoItems>(new Url().update, toDoItem, 200);
}
