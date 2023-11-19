import React, { useState } from 'react';
import Logo from "./Logo";

type Props = {
  currentPage: string;
  setCurrentPage: (page: string) => void;
}

const Navigation: React.FC<Props> = ({setCurrentPage, currentPage}) => {
  const [isOpen, setIsOpen] = useState(false);

  const pages = ["home_page", "station_management_page"];

  const handlePageChange = (page: string) => {
    setCurrentPage(page);
    setIsOpen(false);
  }

  return (
    <nav className="bg-white shadow">
      <div className="max-w-7xl mx-auto px-2 sm:px-4 lg:px-8">
        <div className="relative flex items-center justify-between h-16">
          <div className="absolute inset-y-0 left-0 flex items-center sm:hidden">
            <button onClick={() => setIsOpen(!isOpen)} className="inline-flex items-center justify-center p-2 rounded-md text-gray-400 hover:text-white hover:bg-gray-700 focus:outline-none focus:bg-gray-700 focus:text-white transition duration-150 ease-in-out">
              <svg className="h-6 w-6" stroke="currentColor" fill="none" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d={isOpen ? "M6 18L18 6M6 6l12 12" : "M4 6h16M4 12h16M4 18h16"} />
              </svg>
            </button>
          </div>
          <div className="flex-1 flex items-center justify-center sm:items-stretch sm:justify-start">
            <div className="flex-shrink-0">
              <Logo />
            </div>
            <div className="hidden sm:block sm:ml-6">
              <div className="flex">
                {pages.map((page, index) => (
                  <a href="#" key={index} onClick={() => handlePageChange(page)} className={`mx-4 px-3 py-2 rounded-md text-sm font-medium ${currentPage === page ? 'bg-gray-900 text-white' : 'text-gray-700 hover:text-white hover:bg-gray-700'}`}>{page}</a>
                ))}
              </div>
            </div>
          </div>
        </div>
      </div>
      {isOpen && (
        <div className="sm:hidden">
          <div className="px-2 pt-2 pb-3">
            {pages.map((page, index) => (
              <a href="#" key={index} onClick={() => handlePageChange(page)} className={`block px-3 py-2 rounded-md text-base font-medium ${currentPage === page ? 'bg-gray-900 text-white' : 'text-gray-700 hover:text-white hover:bg-gray-700'}`}>{page}</a>
            ))}
          </div>
        </div>
      )}
    </nav>
  );
}

export default Navigation;