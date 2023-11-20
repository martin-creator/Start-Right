import React, { useState } from "react";
import axios from "axios";
import "tailwindcss/tailwind.css";

function MasterPage() {
  const [farmData, setFarmData] = useState({});
  const [farmId, setFarmId] = useState(null);

  const createFarmAPI = async (data: any) => {
    try {
      const response = await axios.post("/api/v1/farms", data);
      // Process response from API call
    } catch (e: any) {
      // Handle error during API call
    }
  };

  const getFarmsAPI = async () => {
    try {
      const response = await axios.get("/api/v1/farms");
      // Process response from API call
    } catch (e: any) {
      // Handle error during API call
    }
  };

  const updateFarmAPI = async (data: any) => {
    try {
      const response = await axios.put("/api/v1/farms", data);
      // Process response from API call
    } catch (e: any) {
      // Handle error during API call
    }
  };

  const getFarmByIdAPI = async (id: string) => {
    try {
      const response = await axios.get(`/api/v1/farms/${id}`);
      // Process response from API call
    } catch (e: any) {
      // Handle error during API call
    }
  };

  const deleteFarmAPI = async (id: string) => {
    try {
      const response = await axios.delete(`/api/v1/farms/${id}`);
      // Process response from API call
    } catch (e: any) {
      // Handle error during API call
    }
  };

  return (
    <div className="w-full h-full bg-gray-100">
      <section className="flex flex-col items-center justify-center py-10">
        <div className="farm_section flex flex-col items-center justify-center mb-10">
          <h1 className="text-4xl font-bold text-blue-700">Welcome to Our Farm Management!</h1>
          <p className="text-xl">Manage your farms efficiently and effectively.</p>
        </div>
        <div className="farm_management_section flex flex-col items-center justify-center mt-10">
          <h2 className="text-3xl font-bold">Farm Management</h2>
          <form className="flex flex-col items-center justify-center mt-2" onSubmit={(e) => { e.preventDefault(); createFarmAPI(farmData); }}>
            <input type="text" placeholder="Farm Name" className="mt-2 p-2 border rounded w-64" onChange={(e) => setFarmData({ ...farmData, name: e.target.value })} />
            <input type="text" placeholder="Farm Location" className="mt-2 p-2 border rounded w-64" onChange={(e) => setFarmData({ ...farmData, location: e.target.value })} />
            <button type="submit" className="mt-2 p-2 border rounded bg-blue-500 text-white w-64">Create Farm</button>
          </form>
        </div>
      </section>
    </div>
  );
}

export default MasterPage;