import ReactDOM from 'react-dom/client'
import React from 'react'; 
import App from './App.tsx';
import { BrowserRouter } from 'react-router-dom'

ReactDOM.createRoot(document.getElementById('app')).render(
    <BrowserRouter>
        <App />
    </BrowserRouter>
)
console.log('createRoot')
