import React, { useState, useEffect } from 'react';
//import { Routes, Route, Link } from 'react-router-dom';
import {Link } from 'react-router-dom';

function Page() {
    return (
    <div>
        <Link to="/foo" class="ms-2">Home</Link>
        <Link to="/plan" class="ms-2">[ Plan ]</Link>
        <Link to="/todo" class="ms-2">[ Todo ]</Link>
        <Link to="/todo13" class="ms-2">[ Todo13 ]</Link>
        <Link to="/todo16" class="ms-2">[ Todo16 ]</Link>
        <hr />
    </div>
    );
}
export default Page;
