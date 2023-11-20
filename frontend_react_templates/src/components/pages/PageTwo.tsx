import React, { useEffect, useState } from 'react';
import axios from 'axios';
import 'tailwindcss/tailwind.css';

function MasterPage() {
  const [farmData, setFarmData] = useState([]);
  const [farmItem, setFarmItem] = useState(null);
  const [error, setError] = useState(null);

  const fetchFarmData = async (id) => {
    try {
      const response = await axios.get(`/api/v1/farms/${id}`);
      setFarmData(response.data);
    } catch (e) {
      setError(e.message);
    }
  };

  const deleteFarmData = async (id) => {
    try {
      await axios.delete(`/api/v1/farms/${id}`);
      fetchFarmData(id);
    } catch (e) {
      setError(e.message);
    }
  };

  useEffect(() => {
    fetchFarmData();
  }, []);

  return (
    <div className="w-full h-full bg-gray-100 p-8">
      <section className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-4">
        <div className="user_info_section flex flex-col items-center justify-center p-6 bg-white rounded-lg shadow-md">
            <h2 className="text-2xl font-bold mb-2">Farm Information</h2>
            <p className="text-lg text-gray-600">Here you can view your farm information.</p>
        </div>
        <div className="update_info_section flex flex-col items-center justify-center p-6 bg-white rounded-lg shadow-md">
            <h2 className="text-2xl font-bold mb-2">Update Information</h2>
            <p className="text-lg text-gray-600">Update your farm information here.</p>
        </div>
        <div className="delete_farm_section flex flex-col items-center justify-center p-6 bg-white rounded-lg shadow-md">
            <h2 className="text-2xl font-bold mb-2">Delete Farm</h2>
            <p className="text-lg text-gray-600">Delete your farm information here.</p>
        </div>
        <div className="logout_section flex flex-col items-center justify-center p-6 bg-white rounded-lg shadow-md">
            <h2 className="text-2xl font-bold mb-2">Logout</h2>
            <p className="text-lg text-gray-600">Click the button below to securely log out of your account.</p>
        </div>
      </section>
      <section className="flex flex-col items-center justify-center">
        <div className="farm_info_section flex flex-col items-center justify-center">
            <h1 className="text-2xl font-bold">Farm Name</h1>
            <p className="text-lg">Cow Count: </p>
            <p className="text-lg">Goat Count: </p>
            <p className="text-lg">Production: </p>
        </div>
        <div className="update_farm_section flex flex-col items-center justify-center">
            <h2 className="text-xl font-bold">Update Farm Information</h2>
            <form className="flex flex-col items-center justify-center">
                <input type="text" placeholder="Farm Name" className="mb-2"/>
                <input type="number" placeholder="Cow Count" className="mb-2"/>
                <input type="number" placeholder="Goat Count" className="mb-2"/>
                <input type="text" placeholder="Production" className="mb-2"/>
                <button type="submit" className="bg-blue-500 text-white px-4 py-2 rounded">Update</button>
            </form>
        </div>
        <div className="delete_farm_section flex flex-col items-center justify-center">
            <button className="bg-red-500 text-white px-4 py-2 rounded">Delete Farm</button>
        </div>
      </section>
    </div>
  );
}

export default MasterPage;