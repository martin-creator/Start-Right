import React from "react";
import axios from "axios";
import "tailwindcss/tailwind.css";

function MasterPage() {
  const registerAPI = async (data: any) => {
    try {
      const response = await axios.post("/register", data);
      // Process response from API call
    } catch (e: any) {
      // Handle error during API call
    }
  };

  const loginAPI = async (data: any) => {
    try {
      const response = await axios.post("/login", data);
      // Process response from API call
    } catch (e: any) {
      // Handle error during API call
    }
  };

  const externalAPI = async () => {
    try {
      const response = await axios.get("https://ipapi.co/json");
      // Process response from API call
    } catch (e: any) {
      // Handle error during API call
    }
  };

  return (
    <div className="w-full h-full bg-gray-100">
      <section className="flex flex-col items-center justify-center py-10">
        <div className="banner_section flex flex-col items-center justify-center mb-10">
          <h1 className="text-4xl font-bold text-blue-700">Welcome to Our Website</h1>
          <p className="text-lg text-gray-600">This is a platform where you can manage your meetings and accounts efficiently.</p>
        </div>
        <div className="login_section flex flex-col items-center justify-center mb-10">
          <h2 className="text-3xl font-bold text-blue-700">Log In</h2>
          <form className="flex flex-col items-center">
            <input type="text" placeholder="Username" className="my-2 p-2 border rounded w-64"/>
            <input type="password" placeholder="Password" className="my-2 p-2 border rounded w-64"/>
            <button type="submit" className="my-2 p-2 border rounded bg-blue-500 text-white w-64">Log In</button>
          </form>
        </div>
        <div className="meeting_overview_section flex flex-col items-center justify-center mb-10">
          <h2 className="text-3xl font-bold text-blue-700">Recent Meetings</h2>
          <p className="text-lg text-gray-600">Here you can see an overview of your most recent meetings.</p>
        </div>
        <div className="register_section flex flex-col items-center justify-center">
          <h2 className="text-3xl font-bold text-blue-700">Register</h2>
          <form className="flex flex-col items-center">
            <input type="text" placeholder="Username" className="my-2 p-2 border rounded w-64"/>
            <input type="email" placeholder="Email" className="my-2 p-2 border rounded w-64"/>
            <input type="password" placeholder="Password" className="my-2 p-2 border rounded w-64"/>
            <button type="submit" className="my-2 p-2 border rounded bg-blue-500 text-white w-64">Register</button>
          </form>
        </div>
      </section>
    </div>
  );
}

export default MasterPage;