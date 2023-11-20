type Props = {
  currentPage: string;
  setCurrentPage: (page: string) => void;
}

const Footer: React.FC<Props> = ({setCurrentPage, currentPage}) => {
  const pages = ["home_page", "farm_detail_page"];
  const colors = ["#32a852", "#0fa0d1", "#d10fcb"];

  const handleClick = (page: string) => {
    setCurrentPage(page);
  }

  return (
    <div className="fixed bottom-0 w-full bg-gray-200 p-4 flex justify-around items-center md:flex-row flex-col">
      {pages.map((page, index) => (
        <button 
          key={index} 
          onClick={() => handleClick(page)} 
          className={`text-lg font-bold ${currentPage === page ? colors[index % colors.length] : 'text-black'}`}
        >
          {page.replace('_', ' ')}
        </button>
      ))}
    </div>
  )
}

export default Footer;