import ReactDOM from 'react-dom/client'
import React from 'react'; 
import App from './App.tsx';

/* 
function App() {
  return (
  <div>
    <h1 className="text-3xl font-bold text-blue-600">hello!</h1>
    <hr />
    <span>welcome , Rust axum +  React</span>
  </div>
  );
}
*/
ReactDOM.createRoot(document.getElementById('app')).render(
    <App />
)
console.log('createRoot')
