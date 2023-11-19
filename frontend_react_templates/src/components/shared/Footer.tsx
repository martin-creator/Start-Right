type Props = {
  currentPage: string;
  setCurrentPage: (page: string) => void;
}

const Footer: React.FC<Props> = ({setCurrentPage, currentPage}) => {
  const pages = ["home_page", "station_management_page"];
  const colors = ["#1f2937", "#4b5563", "#9ca3af"];

  const handleClick = (page: string) => {
    setCurrentPage(page);
  }

  return (
    <div className="fixed bottom-0 w-full bg-gray-800 text-white p-4">
      <div className="container mx-auto flex justify-around">
        {pages.map((page, index) => (
          <div 
            key={index} 
            onClick={() => handleClick(page)}
            className={`cursor-pointer ${currentPage === page ? 'text-' + colors[index] : ''}`}
          >
            {page.replace('_', ' ')}
          </div>
        ))}
      </div>
    </div>
  )
}

export default Footer;