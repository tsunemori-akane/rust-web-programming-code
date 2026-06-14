import { createRoot } from "react-dom/client";
import React from "react";

const App = () => {
  return (
    <div>
      <h1>Hello, React!</h1>
    </div>
  );
};

const rootElement = document.getElementById("root");
if (!rootElement) {
  throw new Error("Root element not found");
}
const root = createRoot(rootElement);
root.render(<App />);
