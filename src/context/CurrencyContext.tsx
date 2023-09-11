import {
  Dispatch,
  ReactNode,
  SetStateAction,
  createContext,
  useContext,
  useEffect,
  useState,
} from "react";
import axios from "axios";

export interface ICurrencyList {
  baseRate: string;
  dict: ICurrency;
}

export interface ICurrency {
  [symbol: string]: {
    conversionRate: number;
    existingAmount?: number;
    name?: string;
    isSelected: boolean;
  };
}

const initialState: {
  currencies: ICurrencyList;
  setCurrencies: Dispatch<SetStateAction<ICurrencyList>>;
  selectedCurrencies: [];
  setSelectedCurrencies: Dispatch<SetStateAction<[]>>;
} = {
  currencies: { baseRate: "", dict: {} },
  setCurrencies: () => {},
  selectedCurrencies: [],
  setSelectedCurrencies: () => {},
};

const Context = createContext(initialState);

export const useCurrencyContext = () => useContext(Context);

export const CurrencyListProvider = ({ children }: { children: ReactNode }) => {
  const [currencies, setCurrencies] = useState<ICurrencyList>(
    initialState.currencies
  );
  const [selectedCurrencies, setSelectedCurrencies] = useState<[]>([]);

  const fetchCurrencies = async () => {
    try {
      const { data: fiatData } = await axios.get(
        "https://raw.githubusercontent.com/ARK-Builders/ark-exchange-rates/main/fiat-rates.json"
      );
      const { data: cryptoData } = await axios.get(
        "https://raw.githubusercontent.com/ARK-Builders/ark-exchange-rates/main/crypto-rates.json"
      );
      const list: ICurrency = {};
      Object.entries(fiatData.rates).forEach(
        ([currencyCode, conversionRate]) => {
          list[currencyCode] = {
            conversionRate: conversionRate as number,
            isSelected: false,
          };
        }
      );
      cryptoData.forEach(
        (cryptoCurrency: {
          current_price: number;
          symbol: string;
          name: string;
        }) => {
          list[cryptoCurrency.symbol.toUpperCase()] = {
            conversionRate: 1 / cryptoCurrency.current_price as number,
            name: cryptoCurrency.name,
            isSelected: false,
          };
        }
      );
      setCurrencies({ baseRate: fiatData.base, dict: list });
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
