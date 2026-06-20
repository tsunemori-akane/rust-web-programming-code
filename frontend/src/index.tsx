import { createRoot } from "react-dom/client";
import React, { useState } from "react";
import getAll from "./api/get";
import type { ToDoItems } from "./interfaces/toDoItems";

const App = () => {
  const [data, setData] = useState<string | ToDoItems>("");
  const [error, setError] = useState<string | null>(null); // State to store error messages

  React.useEffect(() => {
    const fetchData = async () => {
      const response = await getAll();

      if (response.error) {
        setError(response.error); // Set error if response contains an error
      } else {
        setData(response.data); // Set data if response is successful
      }
    };

    fetchData();
  }, []);

  return (
    <div>
      {error ? (
        <div style={{ color: "red" }}>Error: {error}</div> // Display error if present
      ) : data ? (
        <div>Data loaded: {JSON.stringify(data)}</div>
      ) : (
        <div>Loading...</div>
      )}
    </div>
  );
};

const rootElement = document.getElementById("root");
if (!rootElement) {
  throw new Error("Root element not found");
}
const root = createRoot(rootElement);
root.render(<App />);
