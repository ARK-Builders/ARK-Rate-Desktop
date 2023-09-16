import {
  Dispatch,
  ReactNode,
  SetStateAction,
  createContext,
  useContext,
  useEffect,
  useState,
} from "react";
import { invoke } from "@tauri-apps/api/tauri";

export interface ICurrencyMap {
  [symbol: string]: {
    conversionRate: number;
    existingAmount?: number;
    name?: string;
    isSelected: boolean;
  };
}

const initialState: {
  currencies: ICurrencyMap;
  setCurrencies: Dispatch<SetStateAction<ICurrencyMap>>;
  selectedCurrencies: [];
  setSelectedCurrencies: Dispatch<SetStateAction<[]>>;
} = {
  currencies: {},
  setCurrencies: () => {},
  selectedCurrencies: [],
  setSelectedCurrencies: () => {},
};

const Context = createContext(initialState);

export const useCurrencyContext = () => useContext(Context);

export const CurrencyListProvider = ({ children }: { children: ReactNode }) => {
  const [currencies, setCurrencies] = useState<ICurrencyMap>(
    initialState.currencies
  );
  const [selectedCurrencies, setSelectedCurrencies] = useState<[]>([]);

  const fetchCurrencies = async () => {
    try {
      invoke<Map<String, number>>("get_rates").then((rates) => {
        const list: ICurrencyMap = {};
        Object.entries(rates).forEach(
          ([currencyCode, conversionRate]) => {
            list[currencyCode] = {
              conversionRate: conversionRate as number,
              isSelected: false,
            };
          }
        );

        setCurrencies(list);
      });
    } catch (err) {
      console.error("Problem with fetching currency data: ", err);
    }
  };

  useEffect(() => {
    fetchCurrencies();
  }, []);

  return (
    <Context.Provider
      value={{
        currencies,
        setCurrencies,
        selectedCurrencies,
        setSelectedCurrencies,
      }}
    >
      {children}
    </Context.Provider>
  );
};
