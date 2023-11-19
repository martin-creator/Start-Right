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
          <h1 className="text-4xl font-bold text-blue-700">Welcome to Our Train Station</h1>
          <p className="text-xl">Providing you with the best travel experience</p>
        </div>
        <div className="station_info_section flex flex-col items-center justify-center mt-10">
          <h2 className="text-3xl font-bold">About Our Station</h2>
          <p className="text-lg">Our train station has a rich history and offers a variety of facilities and services to ensure a comfortable journey for all our passengers.</p>
        </div>
        <div className="train_status_section flex flex-col items-center justify-center mt-10">
          <h2 className="text-3xl font-bold">Real-Time Train Status</h2>
          <p className="text-lg">Stay updated with the current status of all trains at our station.</p>
        </div>
        <div className="login_section flex flex-col items-center justify-center mt-10">
          <h2 className="text-3xl font-bold">Student Login</h2>
          <form className="flex flex-col items-center justify-center mt-2">
            <input type="text" placeholder="Username" className="mt-2 p-2 border rounded w-64"/>
            <input type="password" placeholder="Password" className="mt-2 p-2 border rounded w-64"/>
            <button type="submit" className="mt-2 p-2 border rounded bg-blue-500 text-white w-64">Login</button>
          </form>
        </div>
        <div className="register_section flex flex-col items-center justify-center mt-10">
          <h2 className="text-3xl font-bold">New Student Registration</h2>
          <form className="flex flex-col items-center justify-center mt-2">
            <input type="text" placeholder="Full Name" className="mt-2 p-2 border rounded w-64"/>
            <input type="email" placeholder="Email" className="mt-2 p-2 border rounded w-64"/>
            <input type="password" placeholder="Password" className="mt-2 p-2 border rounded w-64"/>
            <button type="submit" className="mt-2 p-2 border rounded bg-blue-500 text-white w-64">Register</button>
          </form>
        </div>
      </section>
    </div>
  );
}

export default MasterPage;