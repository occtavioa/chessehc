import React from "react";
import ReactDOM from "react-dom/client";
import { RouterProvider, createBrowserRouter } from "react-router-dom";
import Home from "./routes/Home";
import TournamentLayout from "./routes/TournamentLayout";
import TournamentData from "./routes/TournamentData";
import Players from "./routes/Players";
import Pairings from "./routes/Pairings";
import Standings from "./routes/Standings";
import Error from "./routes/Error";
import { invoke } from "@tauri-apps/api";
import 'bootstrap/dist/css/bootstrap.min.css';
import "bootstrap-icons/font/bootstrap-icons.css";

const router = createBrowserRouter([
  {
    index: true,
    element: <Home></Home>
  },
  {
    path: "tournament/:path",
    element: <TournamentLayout></TournamentLayout>,
    children: [
      {
        index: true,
        element: <TournamentData></TournamentData>,
        loader: async ({params}) => {
          let {path} = params
          let tournament = await invoke("get_tournament", {path: atob(path)})
          console.log(tournament);
          return tournament
        }
      },
      {
        path: "players",
        element: <Players></Players>,
        loader: async ({params}) => {
          let {path} = params
          let players = await invoke("get_players", {path: atob(path)})
          return players
        },
        action: async ({request, params}) => {
          let {path} = params
          let tournament = await invoke("get_tournament", {path: atob(path)})
          let player = Object.fromEntries(await request.formData())
          player.tournament_id = tournament.id
          player.points = 0.0,
          player.title = player.title === "" ? null : player.title
          player.rating = parseInt(player.rating)
          player.id = 0;
          try {
            await invoke("add_player", {path: atob(path), player: player})
          } catch (error) {
            console.error(error);
          }
          return null
        }
      },
      {
        path: "round/:roundId",
        children: [
          {
            path: "pairings",
            element: <Pairings></Pairings>,
            loader: async ({params}) => {
              let {path, roundId} = params
              let [games, byes, ..._] = await invoke("get_pairings_by_round", {path: atob(path), roundId: parseInt(roundId)})
              let players = await invoke("get_standings_by_round", {path: atob(path), roundId: parseInt(roundId)})
              return {players, games, byes}
            }
          },
          {
            path: "standings",
            element: <Standings></Standings>,
            loader: async ({params}) => {
              let {path, roundId} = params
              let standings = await invoke("get_standings_by_round", {path: atob(path), roundId: parseInt(roundId)})
              let status = await invoke("get_tournament_status", {path: atob(path)})
              return {standings, status}
            }
          }
        ]
      }
    ]
  },
  {
    path: "error",
    element: <Error></Error>
  }
])

ReactDOM.createRoot(document.getElementById("root")).render(
  <React.StrictMode>
    <RouterProvider router={router}></RouterProvider>
  </React.StrictMode>,
);
