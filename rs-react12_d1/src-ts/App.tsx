import { Routes, Route } from 'react-router-dom';
import React from 'react';

import Home from './client/Home';
//import Login from './client/Login';
//import About from './client/about';
import Todo from './client/Todo';
import Todo13 from './client/Todo13';
import Todo16 from './client/Todo16';
import Plan from './client/Plan';

export default function App(){
  return(
  <div className="App">
    <Routes>
      <Route path="/foo" element={<Home />} />
      <Route path="/todo" element={<Todo />} />
      <Route path="/todo13" element={<Todo13 />} />
      <Route path="/todo16" element={<Todo16 />} />
      <Route path="/plan" element={<Plan />} />
    </Routes>
  </div>
  )
}
/*
<Route path="/about" element={<About />} />
<Route path="/login" element={<Login />} />
*/