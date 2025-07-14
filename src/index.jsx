/* @refresh reload */
import { render } from "solid-js/web";
import App from "./App";
import './index.css'
import { Route, Router } from "@solidjs/router";
import Home from "./components/Home/Home";

render(() => (
    <Router>
        <Route path="/" component={Home} />
    </Router>
), document.getElementById("root"));
