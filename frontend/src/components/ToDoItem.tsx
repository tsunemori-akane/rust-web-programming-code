// File: frontend/src/components/ToDoItem.tsx
import React, { useEffect, useState } from 'react';
import { updateToDoItemCall } from "../api/update";
import { deleteToDoItemCall } from "../api/delete";
import { TaskStatus } from "../interfaces/toDoItems";

interface ToDoItemProps {
  title: string;
  status: string;
  passBackResponse: (response: any) => void;
  buttonMessage: string;
  id: number;
}


export const ToDoItem: React.FC<ToDoItemProps> = (
  { title, status, passBackResponse, buttonMessage, id }) => {

  const sendRequest = async () => {
    if (buttonMessage === "edit") {
      await updateToDoItemCall(
        title,
        TaskStatus.DONE,
        id
      ).then(
        response => {
          passBackResponse(response);
        }
      )
    } else {
      await deleteToDoItemCall(title).then(
        response => {
          passBackResponse(response);
        }
      )
    }
  };

  return (
    <div className="itemContainer">
      <p>{title}</p>
      <button className="actionButton"
        onClick={sendRequest}>
        {buttonMessage}
      </button>
    </div>
  );
}