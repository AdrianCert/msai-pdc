import logging
from logging.handlers import TimedRotatingFileHandler

LOGGER_NAME = "snap4frame-backend"

logger = logging.getLogger(LOGGER_NAME)


def config_logger(logger: logging.Logger):
    logger.handlers.clear()

    logger.handlers.append(
        TimedRotatingFileHandler(
            filename="snap4frame-backend.log",
            when="D",
        )
    )


config_logger(logger)
logger.setLevel(logging.DEBUG)

# expose logging functions
debug = logger.debug
info = logger.info
warning = logger.warning
exception = logger.exception
error = logger.error
critical = logger.critical
