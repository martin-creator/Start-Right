import React, { useState } from "react";
import axios from "axios";
import "tailwindcss/tailwind.css";

function MasterPage() {
  const [registerData, setRegisterData] = useState({});
  const [loginData, setLoginData] = useState({});

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
          <h1 className="text-4xl font-bold text-blue-700">Welcome to Our Nutrition and Allergy Tracker!</h1>
          <p className="text-xl">Your health is our priority. Let's start tracking your nutrition requirements and allergies.</p>
        </div>
        <div className="about_section flex flex-col items-center justify-center mt-10">
          <h2 className="text-3xl font-bold">Why Track Nutrition and Allergies?</h2>
          <p className="text-lg">Understanding your nutrition requirements and allergies is crucial for maintaining a healthy lifestyle. Our website provides a simple and efficient way to keep track of your dietary needs.</p>
        </div>
        <div className="login_section flex flex-col items-center justify-center mt-10">
          <h2 className="text-3xl font-bold">Login</h2>
          <form className="flex flex-col items-center justify-center mt-2" onSubmit={(e) => { e.preventDefault(); loginAPI(loginData); }}>
            <input type="text" placeholder="Username" className="mt-2 p-2 border rounded w-64" onChange={(e) => setLoginData({ ...loginData, username: e.target.value })} />
            <input type="password" placeholder="Password" className="mt-2 p-2 border rounded w-64" onChange={(e) => setLoginData({ ...loginData, password: e.target.value })} />
            <button type="submit" className="mt-2 p-2 border rounded bg-blue-500 text-white w-64">Login</button>
          </form>
        </div>
        <div className="register_section flex flex-col items-center justify-center mt-10">
          <h2 className="text-3xl font-bold">Register</h2>
          <form className="flex flex-col items-center justify-center mt-2" onSubmit={(e) => { e.preventDefault(); registerAPI(registerData); }}>
            <input type="text" placeholder="Full Name" className="mt-2 p-2 border rounded w-64" onChange={(e) => setRegisterData({ ...registerData, name: e.target.value })} />
            <input type="email" placeholder="Email" className="mt-2 p-2 border rounded w-64" onChange={(e) => setRegisterData({ ...registerData, email: e.target.value })} />
            <input type="password" placeholder="Password" className="mt-2 p-2 border rounded w-64" onChange={(e) => setRegisterData({ ...registerData, password: e.target.value })} />
            <button type="submit" className="mt-2 p-2 border rounded bg-blue-500 text-white w-64">Register</button>
          </form>
        </div>
      </section>
    </div>
  );
}

export default MasterPage;